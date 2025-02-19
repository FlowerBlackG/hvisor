use crate::{arch::zone::HvArchZoneConfig, config::*};

pub const ROOT_ZONE_DTB_ADDR: u64 = 0x08300000;
pub const ROOT_ZONE_KERNEL_ADDR: u64 = 0x00280000;
pub const ROOT_ZONE_ENTRY: u64 = 0x00280000;
pub const ROOT_ZONE_CPUS: u64 = (1 << 0)|(1 << 1)|(1<<2)|(1<<3) ;

pub const ROOT_ZONE_NAME: &str = "root-linux";
pub const ROOT_ZONE_MEMORY_REGIONS: [HvConfigMemoryRegion; 60] = [
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x0,
        virtual_start: 0x0,
        size: 0x200000,
    }, // ram
    /*
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x0100000,
        virtual_start: 0x0100000,
        size: 0xc0000,
    }, // ram
    */
    
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x0200000,
        virtual_start: 0x0200000,
        size: 0xE6E00000,
    }, // ram
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0xf2000000,
        virtual_start: 0xf2000000,
        size: 0x1000000,
    }, // ram
    
   

    
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0xe7000000,
        virtual_start: 0xe7000000,
        size: 0x7000000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0xee000000,
        virtual_start: 0xee000000,
        size: 0x4000000,
    }, //uart

    
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe000000,
        virtual_start: 0xfe000000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe010000,
        virtual_start: 0xfe010000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe2a0000,
        virtual_start: 0xfe2a0000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe660000,
        virtual_start: 0xfe660000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFD000000,
        virtual_start: 0xFD000000,
        size: 0x200000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFD900000,
        virtual_start: 0xFD900000,
        size: 0x80000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDC20000,
        virtual_start: 0xFDC20000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDC60000,
        virtual_start: 0xFDC60000,
        size: 0x30000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDEA0000,
        virtual_start: 0xFDEA0000,
        size: 0x50000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDC90000,
        virtual_start: 0xFDC90000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDCb0000,
        virtual_start: 0xFDCb0000,
        size: 0x10000,
    }, //uart
    
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDD20000,
        virtual_start: 0xFDD20000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFD400000,
        virtual_start: 0xFD400000,
        size: 0x200000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFD600000,
        virtual_start: 0xFD600000,
        size: 0x100000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFD800000,
        virtual_start: 0xFD800000,
        size: 0x100000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDD90000,
        virtual_start: 0xFDD90000,
        size: 0x10000,
    }, //uart
    //-----------------------------------------------------
    /*
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDf40000,
        virtual_start: 0xFDf40000,
        size: 0x20000,
    }, //uart
   
  */ 
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE400000,
        virtual_start: 0xFE400000,
        size: 0x30000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE600000,
        virtual_start: 0xFE600000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE800000,
        virtual_start: 0xFE800000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE830000,
        virtual_start: 0xFE830000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE0A0000,
        virtual_start: 0xFE0A0000,
        size: 0x20000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE680000,
        virtual_start: 0xFE680000,
        size: 0x60000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE260000,
        virtual_start: 0xFE260000,
        size: 0x30000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE6E0000,
        virtual_start: 0xFE6E0000,
        size: 0x20000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE550000,
        virtual_start: 0xFE550000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE8A0000,
        virtual_start: 0xFE8A0000,
        size: 0x30000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE530000,
        virtual_start: 0xFE530000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE5f0000,
        virtual_start: 0xFE5f0000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x1f0000000,
        virtual_start: 0x1f0000000,
        size: 0x40000000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDD00000,
        virtual_start: 0xFDD00000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDD40000,
        virtual_start: 0xFDD40000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0xFE2b0000,
        virtual_start: 0xFE2b0000,
        size: 0x10000,
    }, //uart
    /*
     //------
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDD00000,
        virtual_start: 0xFDD00000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDD90000,
        virtual_start: 0xFDD90000,
        size: 0x10000,
    }, //uart

    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0xFE130000,
        virtual_start: 0xFE130000,
        size: 0x20000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0xFE130000,
        virtual_start: 0xFE130000,
        size: 0x20000,
    }, //uart
    */
  
    //---------------------------
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE720000,
        virtual_start: 0xFE720000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE710000,
        virtual_start: 0xFE710000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE5E0000,
        virtual_start: 0xFE5E0000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFCC00000,
        virtual_start: 0xFCC00000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe128000,
        virtual_start: 0xfe128000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0xFE100000,
        virtual_start: 0xFE100000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDF80000,
        virtual_start: 0xFDF80000,
        size: 0x20000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDF40000,
        virtual_start: 0xFDF40000,
        size: 0x20000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE750000,
        virtual_start: 0xFE750000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE740000,
        virtual_start: 0xFE740000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDD60000,
        virtual_start: 0xFDD60000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDEf0000,
        virtual_start: 0xFDEf0000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x3c0000000,
        virtual_start: 0x3c0000000,
        size: 0xc00000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFDCA0000,
        virtual_start: 0xFDCA0000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE310000,
        virtual_start: 0xFE310000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE380000,
        virtual_start: 0xFE380000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE150000,
        virtual_start: 0xFE150000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE180000,
        virtual_start: 0xFE180000,
        size: 0x20000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xFE760000,
        virtual_start: 0xFE760000,
        size: 0x20000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe138000,
        virtual_start: 0xfe138000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe148000,
        virtual_start: 0xfe148000,
        size: 0x1000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe820000,
        virtual_start: 0xfe820000,
        size: 0x10000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe840000,
        virtual_start: 0xfe840000,
        size: 0x10000,
    }, //uart
];
/*
pub const ROOT_ZONE_MEMORY_REGIONS: [HvConfigMemoryRegion; 6] = [
    /*HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x00200000,
        virtual_start: 0x00200000,
        size: 0x08200000,
    }, // ram
    */
    
    
    
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x09400000,
        virtual_start: 0x09400000,
        size: 0xE6C00000,
    }, // ram
    
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe660000,
        virtual_start: 0xfe660000,
        size: 0x4000,
    }, //uart
    
    
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe530000,
        virtual_start: 0xfe530000,
        size: 0x4000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe540000,
        virtual_start: 0xfe540000,
        size: 0x4000,
    }, //uart
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe550000,
        virtual_start: 0xfe550000,
        size: 0x4000,
    },
    
    
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0xfe5f0000,
        virtual_start: 0xfe5f0000,
        size: 0x1000,
    }, //uart
   
    
];
*/

pub const ROOT_ZONE_IRQS: [u32; 2] = [
   0x27, 0x76];

pub const ROOT_ARCH_ZONE_CONFIG: HvArchZoneConfig = HvArchZoneConfig {
    gicd_base: 0xfd400000,
    gicd_size: 0x10000,
    gicr_base: 0xfd460000,
    gicr_size: 0xc0000,
    gicc_base: 0,
    gicc_size: 0,
    gicc_offset: 0,
    gich_base: 0,
    gich_size: 0,
    gicv_base: 0,
    gicv_size: 0,
    gits_base: 0,
    gits_size: 0,
};

pub const ROOT_ZONE_IVC_CONFIG: [HvIvcConfig; 0] = [];