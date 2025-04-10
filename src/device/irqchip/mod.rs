use crate::arch::zone::HvArchZoneConfig;
use crate::zone::Zone;

#[cfg(all(feature = "gicv2", target_arch = "aarch64"))]
pub mod gicv2;
#[cfg(all(feature = "gicv2", target_arch = "aarch64"))]
pub use gicv2::{
    gic::inject_irq, gicd::set_ispender, percpu_init, primary_init_early, primary_init_late,
    vgic::set_sgi_irq,
};

#[cfg(all(feature = "gicv3", target_arch = "aarch64"))]
pub mod gicv3;
#[cfg(all(feature = "gicv3", target_arch = "aarch64"))]
pub use gicv3::{
    gicd::set_ispender, inject_irq, percpu_init, primary_init_early, primary_init_late,
};

#[cfg(target_arch = "riscv64")]
#[cfg(feature = "plic")]
pub mod plic;

#[cfg(target_arch = "riscv64")]
#[cfg(feature = "aia")]
pub mod aia;

pub fn gic_handle_irq() {
    #[cfg(all(feature = "gicv2", target_arch = "aarch64"))]
    gicv2::gic::gicv2_handle_irq();
    #[cfg(all(feature = "gicv3", target_arch = "aarch64"))]
    gicv3::gicv3_handle_irq_el1();
}

impl Zone {
    pub fn mmio_init(&mut self, hv_config: &HvArchZoneConfig) {
        #[cfg(all(feature = "gicv2", target_arch = "aarch64"))]
        {
            self.vgicv2_mmio_init(hv_config);
            self.vgicv2_remap_init(hv_config);
        }
        #[cfg(all(feature = "gicv3", target_arch = "aarch64"))]
        {
            self.vgicv3_mmio_init(hv_config);
        }
    }
}

#[cfg(target_arch = "riscv64")]
#[cfg(feature = "plic")]
pub use plic::{
    host_plic, inject_irq, percpu_init, primary_init_early, primary_init_late,
    vplic_global_emul_handler, vplic_hart_emul_handler,
};

#[cfg(target_arch = "riscv64")]
#[cfg(feature = "aia")]
pub use aia::aplic::{
    host_aplic, inject_irq, percpu_init, primary_init_early, primary_init_late, vaplic_emul_handler,
};

#[cfg(target_arch = "loongarch64")]
pub mod ls7a2000;

#[cfg(target_arch = "loongarch64")]
pub use ls7a2000::{inject_irq, percpu_init, primary_init_early, primary_init_late};
