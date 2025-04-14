use core::arch::asm;

pub trait Thread: Sized {
    fn current() -> *mut Self {
        let mut p;

        unsafe {
            asm!("mov {}, gs:[0]", out(reg) p, options(readonly, pure, preserves_flags, nostack))
        };

        p
    }
    
    fn ret(&self, i: usize) -> usize;
}