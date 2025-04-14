use crate::Kernel;
use crate::ucred::Ucred;

#[repr(C)]
pub struct Thread {
    pad1: [u8; 0x130],
    cred: *mut Ucred,
    pad2: [u8; 0x260],
    ret: [usize; 2],
}

impl rsf::thread::Thread<Kernel> for Thread {
    fn cred(&self) -> *mut Ucred {
        self.cred
    }

    fn ret(&self, i: usize) -> usize {
        self.ret[i]
    }
}