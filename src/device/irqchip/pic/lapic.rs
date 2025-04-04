use crate::{
    arch::{
        cpu::this_cpu_id,
        idt::IdtVector,
        ipi,
        msr::Msr::{self, *},
    },
    error::HvResult,
    percpu::this_cpu_data,
};
use bit_field::BitField;
use core::{ops::Range, u32};
use x2apic::lapic::{LocalApic, LocalApicBuilder, TimerMode};

pub struct VirtLocalApic {
    pub phys_lapic: LocalApic,
    pub virt_timer_vector: u8,
    virt_lvt_timer_bits: u32,
}

impl VirtLocalApic {
    pub fn new() -> Self {
        let mut lapic = LocalApicBuilder::new()
            .timer_vector(IdtVector::APIC_TIMER_VECTOR as _)
            .error_vector(IdtVector::APIC_ERROR_VECTOR as _)
            .spurious_vector(IdtVector::APIC_SPURIOUS_VECTOR as _)
            .build()
            .unwrap();

        unsafe {
            lapic.enable();
            lapic.disable_timer();
        }

        Self {
            phys_lapic: lapic,
            virt_timer_vector: 0,
            virt_lvt_timer_bits: (1 << 16) as _, // masked
        }
    }

    pub const fn msr_range() -> Range<u32> {
        0x800..0x840
    }

    pub fn phys_local_apic<'a>() -> &'a mut LocalApic {
        &mut this_cpu_data().arch_cpu.virt_lapic.phys_lapic
    }

    pub fn rdmsr(&mut self, msr: Msr) -> HvResult<u64> {
        match msr {
            IA32_X2APIC_APICID => Ok(this_cpu_id() as u64),
            IA32_X2APIC_LDR => Ok(this_cpu_id() as u64), // logical apic id
            IA32_X2APIC_LVT_TIMER => Ok(self.virt_lvt_timer_bits as _),
            _ => hv_result_err!(ENOSYS),
        }
    }

    pub fn wrmsr(&mut self, msr: Msr, value: u64) -> HvResult {
        match msr {
            IA32_X2APIC_ICR => {
                // info!("ICR value: {:x}", value);
                ipi::send_ipi(value);
                Ok(())
            }
            IA32_X2APIC_LVT_TIMER => {
                self.virt_lvt_timer_bits = value as u32;
                self.virt_timer_vector = value.get_bits(0..=7) as _;
                unsafe {
                    self.phys_lapic
                        .set_timer_mode(match value.get_bits(17..19) {
                            0 => TimerMode::OneShot,
                            1 => TimerMode::Periodic,
                            _ => TimerMode::TscDeadline,
                        });
                    if value.get_bit(16) {
                        self.phys_lapic.disable_timer();
                    } else {
                        self.phys_lapic.enable_timer();
                    }
                }
                Ok(())
            }
            _ => hv_result_err!(ENOSYS),
        }
    }
}
