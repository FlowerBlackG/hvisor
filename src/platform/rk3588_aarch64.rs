use crate::{arch::zone::HvArchZoneConfig, config::*};

pub const ROOT_ZONE_DTB_ADDR: u64 = 0x10000000;
pub const ROOT_ZONE_KERNEL_ADDR: u64 = 0x09400000;
pub const ROOT_ZONE_ENTRY: u64 = 0x09400000;
pub const ROOT_ZONE_CPUS: u64 = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3);

pub const ROOT_ZONE_NAME: &str = "root-linux";

pub const ROOT_ZONE_MEMORY_REGIONS: [HvConfigMemoryRegion; 1] = [
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x0,
        virtual_start: 0x0,
        size: 0x7ffffff000,
    }
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_RAM,
    //     physical_start: 0x0000000000200000,
    //     virtual_start: 0x0000000000200000,
    //     size: 0x00000000083fffff - 0x0000000000200000 + 1,
    // }, // ram
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_RAM,
    //     physical_start: 0x0000000009400000,
    //     virtual_start: 0x0000000009400000,
    //     size: 0x00000000efffffff - 0x0000000009400000 + 1,
    // }, // ram
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_RAM,
    //     physical_start: 0x0000000100000000,
    //     virtual_start: 0x0000000100000000,
    //     size: 0x00000003fbffffff - 0x0000000100000000 + 1,
    // }, // ram
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_RAM,
    //     physical_start: 0x00000003fc500000,
    //     virtual_start: 0x00000003fc500000,
    //     size: 0x00000003ffefffff - 0x00000003fc500000 + 1,
    // }, // ram
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_RAM,
    //     physical_start: 0x00000004f0000000,
    //     virtual_start: 0x00000004f0000000,
    //     size: 0x00000004ffffffff - 0x00000004f0000000 + 1,
    // }, // ram
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_IO,
    //     physical_start: 0xfeb50000,
    //     virtual_start: 0xfeb50000,
    //     size: 0x1000,
    // }, // uart
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_IO,
    //     physical_start: 0xfd000000,
    //     virtual_start: 0xfd000000,
    //     size: 0x1000000,
    // },
    // HvConfigMemoryRegion {
    //     mem_type: MEM_TYPE_IO,
    //     physical_start: 0x100000,
    //     virtual_start: 0x100000,
    //     size: 0x100000,
    // }
];

pub const ROOT_ZONE_IRQS: [u32; 1] = [
    0x76];

pub const ROOT_ARCH_ZONE_CONFIG: HvArchZoneConfig = HvArchZoneConfig {
    gicd_base: 0xfe600000,
    gicd_size: 0x10000,
    gicr_base: 0xfe680000,
    gicr_size: 0x100000,
};
