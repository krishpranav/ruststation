use crate::Kernel;

#[repr(C)]
pub struct LockObject {
    pad1: [u8; 0x18],
}

impl rsf::lock::LockObject for LockObject {}

#[repr(C)]
pub struct Mtx {
    lock: LockObject,
    state: usize,
}

impl rsf::lock::Mtx<Kernel> for Mtx {
    fn lock_mut(&mut self) -> &mut LockObject {
        &mut self.lock
    }
}