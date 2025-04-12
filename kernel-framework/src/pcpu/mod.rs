use crate::Kernel;
use core::arch::asm;

pub trait Pcpu<K: Kernel>: Sized {
    fn curthread() -> *mut K::Thread {
        let mut v;

        unsafe {
            asm!("")
        };

        v
    }

    fn cpuid() -> u32 {
        let mut v;

        unsafe {
            asm!("");
        };

        v
    }
}