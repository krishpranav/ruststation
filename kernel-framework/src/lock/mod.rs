use crate::Kernel;

pub trait LockObject: Sized {}

pub trait Mtx<K: Kernel>: Sized {
    fn lock_mut(&mut self) -> &mut K::LockObject;
}

pub struct MtxLock<K: Kernel> {
    kern: K,
    mtx: *mut K::Mtx,
}

impl<K: Kernel> MtxLock<K> {
    pub unsafe fn new(kern: K, mtx: *mut K::Mtx) -> Self {
        unsafe { kern.mtx_lock_flags(mtx, 0, c"".as_ptr(), 0) };
        Self { kern, mtx }
    }
}

impl<K: Kernel> Drop for MtxLock<K> {
    fn drop(&mut self) {
        unsafe { self.kern.mtx_unlock_flags(self.mtx, 0, c"".as_ptr(), 0) };
    }
}