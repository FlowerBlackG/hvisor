use crate::{
    arch::acpi,
    memory::{Frame, HostPhysAddr},
    zone::this_zone_id,
};
use ::acpi::{mcfg::Mcfg, sdt::Signature};
use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use bit_field::BitField;
use core::{arch::asm, hint::spin_loop, mem::size_of, usize};
use dma_remap_reg::*;
use spin::{Mutex, Once};
use x86_64::instructions::port::Port;

const IR_ENTRY_CNT: usize = 256;

// I/O ports
pub const PCI_CONFIG_ADDR: u16 = 0xcf8;
pub const PCI_CONFIG_DATA: u16 = 0xcfc;
const PCI_CONFIG_ENABLE: u32 = 0x80000000;

const ROOT_TABLE_ENTRY_SIZE: usize = 16;
const CONTEXT_TABLE_ENTRY_SIZE: usize = 16;

//  DMA-remapping registers

mod dma_remap_reg {
    /// Extended Capability Register
    pub const DMAR_ECAP_REG: usize = 0x10;
    /// Global Command Register
    pub const DMAR_GCMD_REG: usize = 0x18;
    /// Global Status Register
    pub const DMAR_GSTS_REG: usize = 0x1c;
    /// Root Table Address Register
    pub const DMAR_RTADDR_REG: usize = 0x20;
    /// Fault Event Control Register
    pub const DMAR_FECTL_REG: usize = 0x38;
    /// Invalidation Queue Tail Register
    pub const DMAR_IQT_REG: usize = 0x88;
    /// Invalidation Queue Address Register
    pub const DMAR_IQA_REG: usize = 0x90;
    /// Interrupt Remapping Table Address Register
    pub const DMAR_IRTA_REG: usize = 0xb8;
}

static DRHD_UNITS: Once<Vec<Mutex<Drhd>>> = Once::new();

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct EcapFlags: u64 {
        ///  Extended Interrupt Mode
        const EIM = 1 << 4;
        ///  Interrupt Remapping Support
        const IR = 1 << 3;
        ///  Queued Invalidation Support
        const QI = 1 << 1;
    }

    #[derive(Clone, Copy, Debug)]
    pub struct GstsFlags: u32 {
        /// Translation Enable Status
        const TES = 1 << 31;
        /// Root Table Pointer Status
        const RTPS = 1 << 30;
        /// Queue Invalidation Enable Status
        const QIES = 1 << 26;
        /// Interrupt Remapping Enable Status
        const IRES = 1 << 25;
        /// Interrupt Remap Table Pointer Status
        const IRTPS = 1 << 24;
    }

    #[derive(Clone, Copy, Debug)]
    pub struct GcmdFlags: u32 {
        /// Translation Enable
        const TE = 1 << 31;
        /// Set Root Table Pointer
        const SRTP = 1 << 30;
        /// Queue Invalidation Enable
        const QIE = 1 << 26;
        /// Interrupt Remapping Enable
        const IRE = 1 << 25;
        /// Set Interrupt Remap Table Pointer
        const SIRTP = 1 << 24;
    }
}

numeric_enum_macro::numeric_enum! {
#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum DeviceScopeType {
    NotUsed = 0x00,
    PciEndpointDevice = 0x01,
    PciSubHierarchy = 0x02,
    IoApic = 0x03,
    MsiCapableHpet = 0x04,
    AcpiNamespaceDevice = 0x05
}
}

#[derive(Clone, Debug)]
struct DeviceScope {
    scope_type: DeviceScopeType,
    id: u8,
    bus: u8,
    dev_func: u8,
}

#[derive(Debug)]
struct Drhd {
    flags: u8,
    segment: u16,
    reg_hpa: usize,
    scopes: Vec<DeviceScope>,

    root_table: Frame,
    context_tables: BTreeMap<u8, Frame>,
    qi_queue: Frame,
    ir_table: Frame,
    /// cache value of DMAR_GCMD_REG
    gcmd: GcmdFlags,
}

impl Drhd {
    fn activate(&mut self) {
        self.activate_dma_translation();
    }

    fn activate_dma_translation(&mut self) {
        if !self.gcmd.contains(GcmdFlags::TE) {
            self.gcmd |= GcmdFlags::TE;
            self.mmio_write_u32(DMAR_GCMD_REG, self.gcmd.bits());

            self.wait(GstsFlags::TES, false);
        }
    }

    fn activate_interrupt_remapping(&mut self) {
        if !self.gcmd.contains(GcmdFlags::IRE) {
            self.gcmd |= GcmdFlags::IRE;
            self.mmio_write_u32(DMAR_GCMD_REG, self.gcmd.bits());

            self.wait(GstsFlags::IRES, false);
        }
    }

    fn activate_qi(&mut self) {
        let qi_queue_hpa = self.qi_queue.start_paddr();
        self.mmio_write_u64(DMAR_IQA_REG, qi_queue_hpa as u64);
        self.mmio_write_u32(DMAR_IQT_REG, 0);

        if !self.gcmd.contains(GcmdFlags::QIE) {
            self.gcmd |= GcmdFlags::QIE;

            self.mmio_write_u32(DMAR_GCMD_REG, self.gcmd.bits());

            self.wait(GstsFlags::QIES, false);
        }
    }

    fn add_context_entry(&mut self, bus: u8, dev_func: u8, zone_s2pt_hpa: HostPhysAddr) {
        let root_entry_hpa = self.root_table.start_paddr() + (bus as usize) * ROOT_TABLE_ENTRY_SIZE;
        let root_entry_low = unsafe { &mut *(root_entry_hpa as *mut u64) };

        // context table not present
        if !root_entry_low.get_bit(0) {
            let context_table = Frame::new_zero().unwrap();
            let context_table_hpa = context_table.start_paddr();

            // set context-table pointer
            root_entry_low.set_bits(12..=63, context_table_hpa.get_bits(12..=63) as _);
            // set present
            root_entry_low.set_bit(0, true);

            flush_cache_range(root_entry_hpa, ROOT_TABLE_ENTRY_SIZE);
            self.context_tables.insert(bus, context_table);
        }

        let context_table_hpa = self.context_tables.get(&bus).unwrap().start_paddr();
        let context_entry_hpa = context_table_hpa + (dev_func as usize) * CONTEXT_TABLE_ENTRY_SIZE;
        let context_entry = unsafe { &mut *(context_entry_hpa as *mut u128) };

        // s2pt not present
        if !context_entry.get_bit(0) {
            // address width: 010b (48bit 4-level page table)
            context_entry.set_bits(64..=66, 0b010);
            // domain identifier: zone id
            context_entry.set_bits(72..=87, this_zone_id() as _);
            // second stage page translation pointer
            context_entry.set_bits(12..=63, zone_s2pt_hpa.get_bits(12..=63) as _);
            // present
            context_entry.set_bit(0, true);

            flush_cache_range(context_entry_hpa, CONTEXT_TABLE_ENTRY_SIZE);
        }
    }

    fn add_interrupt_table_entry(&mut self, irq: u32) {
        assert!(irq < (IR_ENTRY_CNT as u32));

        let ir_table_hpa = self.ir_table.start_paddr();
        let irte_hpa = ir_table_hpa + (irq as usize) * size_of::<u128>();
        let irte_ptr = irte_hpa as *mut u128;
        let mut irte: u128 = 0;

        // present
        irte.set_bit(0, true);
        // irte mode: remap
        irte.set_bit(15, false);
        // vector
        irte.set_bits(16..=23, irq as _);
        // dest id
        irte.set_bits(32..=63, 0);

        unsafe { *irte_ptr = irte };
        flush_cache_range(irte_hpa, size_of::<u128>());

        // TODO: iec
    }

    fn check_capability(&mut self) {
        let ecap = EcapFlags::from_bits_truncate(self.mmio_read_u64(DMAR_ECAP_REG));
        info!("ecap: {:x?}", ecap);
        assert!(ecap.contains(EcapFlags::EIM | EcapFlags::IR | EcapFlags::QI));
    }

    fn init(&mut self) {
        self.check_capability();
        self.set_interrupt();
        self.set_root_table();
        self.activate_qi();

        // self.set_interrupt_remap_table();
        /* for irq in 0..IR_ENTRY_CNT {
            self.add_interrupt_table_entry(irq as _);
        } */
        // self.activate_interrupt_remapping();
    }

    fn set_interrupt(&mut self) {
        self.mmio_write_u32(DMAR_FECTL_REG, 0);
    }

    fn set_interrupt_remap_table(&mut self) {
        // bit 12-63: ir table address
        // bit 11: x2apic mode active
        // bit 0-3: X, where 2 ^ (X + 1) == number of entries
        let address: u64 =
            (self.ir_table.start_paddr() as u64) | (1 << 11) | ((IR_ENTRY_CNT.ilog2() - 1) as u64);

        self.mmio_write_u64(DMAR_IRTA_REG, address);
        self.mmio_write_u32(DMAR_GCMD_REG, (self.gcmd | GcmdFlags::SIRTP).bits());

        self.wait(GstsFlags::IRTPS, false);
    }

    fn set_root_table(&mut self) {
        self.mmio_write_u64(DMAR_RTADDR_REG, self.root_table.start_paddr() as _);
        self.mmio_write_u32(DMAR_GCMD_REG, (self.gcmd | GcmdFlags::SRTP).bits());

        self.wait(GstsFlags::RTPS, false);
    }

    fn update_dma_translation_tables(&mut self, zone_s2pt_hpa: HostPhysAddr) {
        let bdfs: Vec<(u8, u8)> = self
            .scopes
            .iter()
            .filter(|scope| scope.scope_type == DeviceScopeType::PciEndpointDevice)
            .map(|scope| (scope.bus, scope.dev_func))
            .collect();

        for (bus, dev_func) in bdfs {
            self.add_context_entry(bus, dev_func, zone_s2pt_hpa);
        }
    }

    fn wait(&mut self, mask: GstsFlags, cond: bool) {
        loop {
            spin_loop();
            if GstsFlags::from_bits_truncate(self.mmio_read_u32(DMAR_GSTS_REG)).contains(mask)
                != cond
            {
                break;
            }
        }
    }

    fn mmio_read_u32(&self, reg: usize) -> u32 {
        unsafe { *((self.reg_hpa + reg) as *const u32) }
    }

    fn mmio_read_u64(&self, reg: usize) -> u64 {
        unsafe { *((self.reg_hpa + reg) as *const u64) }
    }

    fn mmio_write_u32(&self, reg: usize, value: u32) {
        unsafe { *((self.reg_hpa + reg) as *mut u32) = value };
    }

    fn mmio_write_u64(&self, reg: usize, value: u64) {
        unsafe { *((self.reg_hpa + reg) as *mut u64) = value };
    }
}

fn get_secondary_bus(bus: u8, dev: u8, func: u8) -> u8 {
    unsafe {
        Port::<u32>::new(PCI_CONFIG_ADDR).write(
            PCI_CONFIG_ENABLE
                | ((bus as u32) << 16)
                | ((dev as u32) << 11)
                | ((func as u32) << 8)
                | 0x18,
        );
        let data = Port::<u32>::new(PCI_CONFIG_DATA).read();
        ((data >> 8) & 0xff) as u8
    }
}

pub fn parse_root_drhds() -> Vec<Mutex<Drhd>> {
    let mut drhds: Vec<Mutex<Drhd>> = Vec::new();

    let dmar = acpi::root_get_table(&Signature::DMAR).unwrap();
    let mut cur: usize = 48; // start offset of remapping structures
    let len = dmar.get_len();

    while cur < len {
        let struct_type = dmar.get_u16(cur);
        let struct_len = dmar.get_u16(cur + 2) as usize;

        // drhd
        if struct_type == 0 {
            let mut drhd = Drhd {
                flags: dmar.get_u8(cur + 4),
                segment: dmar.get_u16(cur + 6),
                reg_hpa: dmar.get_u64(cur + 8) as usize,
                scopes: Vec::new(),

                root_table: Frame::new_zero().unwrap(),
                context_tables: BTreeMap::new(),
                qi_queue: Frame::new_zero().unwrap(),
                ir_table: Frame::new_zero().unwrap(),
                gcmd: GcmdFlags::empty(),
            };

            let mut scope_cur = cur + 16; // start offset of device scopes
                                          // device scopes
            while scope_cur < cur + struct_len {
                let scope_len = dmar.get_u8(scope_cur + 1) as usize;

                let mut bus = dmar.get_u8(scope_cur + 5);
                let mut path_cur = scope_cur + 6;
                let mut dev = dmar.get_u8(path_cur);
                let mut func = dmar.get_u8(path_cur + 1);

                // info!("bdf: {:x} {:x} {:x}", bus, dev, func);

                path_cur += 2;
                while path_cur < scope_cur + scope_len {
                    bus = get_secondary_bus(bus, dev, func);
                    dev = dmar.get_u8(path_cur);
                    func = dmar.get_u8(path_cur + 1);
                    // info!("bdf: {:x} {:x} {:x}", bus, dev, func);
                    path_cur += 2;
                }

                let mut scope = DeviceScope {
                    scope_type: DeviceScopeType::try_from(dmar.get_u8(scope_cur + 0)).unwrap(),
                    id: dmar.get_u8(scope_cur + 4),
                    bus,
                    dev_func: (dev << 3) | func,
                };
                info!("{:x?}", scope);
                drhd.scopes.push(scope);

                scope_cur += scope_len;
            }

            drhds.push(Mutex::new(drhd));
        } else {
        }

        cur += struct_len;
    }

    drhds
}

pub fn init() {
    DRHD_UNITS.call_once(|| parse_root_drhds());
    for unit in DRHD_UNITS.get().unwrap().iter() {
        unit.lock().init();
    }

    init_msi_cap_hpa_space();
}

pub fn update_dma_translation_tables(zone_s2pt_hpa: HostPhysAddr) {
    for unit in DRHD_UNITS.get().unwrap().iter() {
        unit.lock().update_dma_translation_tables(zone_s2pt_hpa);
    }
}

/// should be called after gpm is activated
pub fn activate() {
    for unit in DRHD_UNITS.get().unwrap().iter() {
        unit.lock().activate();
    }
}

fn flush_cache_range(hpa: usize, size: usize) {
    let mut i = 0usize;
    while i < size {
        unsafe { asm!("clflushopt [{addr}]", addr = in(reg) hpa + i) };
        i += 64;
    }
}

fn init_msi_cap_hpa_space() {
    let bytes = acpi::root_get_table(&Signature::MCFG)
        .unwrap()
        .get_bytes()
        .clone();
    let mcfg = unsafe { &*(bytes.as_ptr() as *const Mcfg) };

    for unit in DRHD_UNITS.get().unwrap().iter() {
        let drhd = unit.lock();

        for entry in mcfg.entries() {
            if entry.pci_segment_group != drhd.segment {
                break;
            }
            let bus_range = entry.bus_number_start..=entry.bus_number_end;

            for scope in drhd.scopes.iter() {
                if scope.scope_type != DeviceScopeType::PciEndpointDevice {
                    continue;
                }

                if !bus_range.contains(&scope.bus) {
                    continue;
                }

                let config_space_hpa = (entry.base_address as usize)
                    + ((scope.bus as usize) << 20)
                    + ((scope.dev_func as usize) << 12);
                let mut cap_pointer = unsafe { *((config_space_hpa + 0x34) as *const u8) } as usize;

                info!("dev_fun: {:x}", scope.dev_func);
                while cap_pointer != 0 {
                    let cap_hpa = config_space_hpa + cap_pointer;
                    let cap_id = unsafe { *(cap_hpa as *const u8) };

                    info!("cap id: {:x}, hpa: {:x}", cap_id, cap_hpa);
                    cap_pointer = unsafe { *((cap_hpa + 1) as *const u8) } as usize;
                }
            }
        }
    }
}
