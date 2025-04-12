use crate::Kernel;
use core::arch::asm;

pub trait Pcpu<K: Kernel>: Sized {
    fn curthread() -> *mut K::Thread {
        let mut v;

        unsafe {
            asm!("mov {}, gs:[0]", out(reg) v, options(readonly, pure, preserves_flags, nostack))
        };

        v
    }

    fn cpuid() -> u32 {
        let mut v;

        unsafe {
            asm!("mov {:e}, gs:[0x34]", out(reg) v, options(readonly, pure, preserves_flags, nostack))
        };

        v
    }
}