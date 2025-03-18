use crate::{
    device::irqchip::inject_vector,
    error::HvResult,
    event,
    percpu::{this_cpu_data, this_zone, CpuSet},
};
use alloc::{collections::vec_deque::VecDeque, vec::Vec};
use bit_field::BitField;
use spin::{Mutex, Once};

use super::{cpu::this_cpu_id, idt::IdtVector};

#[allow(non_snake_case)]
pub mod IpiDeliveryMode {
    pub const FIXED: u8 = 0;
    pub const NMI: u8 = 4;
    pub const INIT: u8 = 5;
    pub const START_UP: u8 = 6;
}

#[allow(non_snake_case)]
pub mod IpiDestShorthand {
    pub const NO_SHORTHAND: u8 = 0;
    pub const SELF: u8 = 1;
    pub const ALL_INCLUDING_SELF: u8 = 2;
    pub const ALL_EXCLUDING_SELF: u8 = 3;
}

pub struct IpiInfo {
    pub start_up_addr: usize,
    pub has_start_up: bool,
}

impl IpiInfo {
    fn new() -> Self {
        Self {
            start_up_addr: 0,
            has_start_up: false,
        }
    }
}

static IPI_MANAGER: Once<IpiManager> = Once::new();
struct IpiManager {
    pub inner: Vec<Mutex<IpiInfo>>,
}

impl IpiManager {
    fn new(max_cpus: usize) -> Self {
        let mut vs = vec![];
        for _ in 0..max_cpus {
            let v = Mutex::new(IpiInfo::new());
            vs.push(v)
        }
        Self { inner: vs }
    }

    fn get_ipi_info<'a>(&'a self, cpu: usize) -> Option<&'a Mutex<IpiInfo>> {
        self.inner.get(cpu)
    }
}

pub fn init(max_cpus: usize) {
    IPI_MANAGER.call_once(|| IpiManager::new(max_cpus));
}

pub fn get_ipi_info<'a>(cpu: usize) -> Option<&'a Mutex<IpiInfo>> {
    IPI_MANAGER.get().unwrap().get_ipi_info(cpu)
}

pub fn send_ipi(value: u64) -> HvResult {
    let vector = value.get_bits(0..=7) as u8;
    let delivery_mode: u8 = value.get_bits(8..=10) as u8;
    let dest_shorthand = value.get_bits(18..=19) as u8;
    let dest = value.get_bits(32..=39) as usize;
    let cnt = value.get_bits(40..=63) as u32;

    let mut cpu_set = this_zone().read().cpu_set;
    let cpu_id = this_cpu_id();
    let mut dest_set = CpuSet::new(cpu_set.max_cpu_id, 0);

    match dest_shorthand {
        IpiDestShorthand::NO_SHORTHAND => {
            dest_set.set_bit(dest);
        }
        IpiDestShorthand::SELF => {
            dest_set.set_bit(cpu_id);
        }
        IpiDestShorthand::ALL_INCLUDING_SELF => {
            dest_set = cpu_set;
        }
        IpiDestShorthand::ALL_EXCLUDING_SELF => {
            dest_set = cpu_set;
            dest_set.clear_bit(cpu_id);
        }
        _ => {}
    }

    dest_set.iter().for_each(|dest| {
        match delivery_mode {
            IpiDeliveryMode::FIXED => {
                inject_vector(dest, vector, None, true);
                arch_send_event(dest as _, IdtVector::VIRT_IPI_VECTOR as _);
            }
            IpiDeliveryMode::NMI => {
                inject_vector(dest, 2, None, true);
                arch_send_event(dest as _, IdtVector::VIRT_IPI_VECTOR as _);
            }
            IpiDeliveryMode::INIT => {}
            IpiDeliveryMode::START_UP => {
                // TODO: target
                let mut ipi_info = get_ipi_info(dest).unwrap().lock();
                if !ipi_info.has_start_up {
                    // we only start up once
                    ipi_info.has_start_up = true;
                    ipi_info.start_up_addr = (vector as usize) << 12;
                    event::send_event(
                        dest,
                        IdtVector::VIRT_IPI_VECTOR as _,
                        event::IPI_EVENT_WAKEUP,
                    );
                }
            }
            _ => {}
        }
    });

    Ok(())
}

pub fn arch_send_event(dest: u64, vector: u64) {
    unsafe {
        this_cpu_data()
            .arch_cpu
            .virt_lapic
            .phys_lapic
            .send_ipi(vector as _, dest as _)
    };
}

pub fn handle_virt_ipi() {
    // this may never return!
    if event::check_events() {
        return;
    }

    // inject ipi
    /*let mut vectors = &mut get_ipi_info(this_cpu_id()).unwrap().lock().fixed_vectors;
    if vectors.len() > 1 {
        // info!("handle_virt_ipi vectors len: {:x}", vectors.len());
    }

    while vectors.len() != 0 {
        if let Some(vector) = vectors.pop_front() {
            // info!("handle_virt_ipi vector: {:x}", vector);
            this_cpu_data()
                .arch_cpu
                .virt_lapic
                .inject_event((vector & 0xff) as u8, None);
        }
    }*/
}
