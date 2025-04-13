use crate::Kernel;
use crate::pcpu::Pcpu;
use core::ffi::c_int;
use core::num::NonZero;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicU32, Ordering, fence};

pub trait File: Sized {
    fn refcnt(&self) -> &AtomicU32;
}

pub struct OwnedFile<K: Kernel> {
    kern: K,
    file: *mut K::File,
}

impl<K: Kernel> OwnedFile<K> {
    pub unsafe fn new(kern: K, file: *mut K::File) -> Self {
        Self { kern, file }
    }

    pub fn from_fd(kern: K, fd: c_int) -> Result<Self, NonZero<c_int>> {
        let td = K::Pcpu::curthread();
        let mut fp = null_mut();
        let errno = unsafe { kern.fget(td, fd, &mut fp, 0, null_mut()) };

        match NonZero::new(errno) {
            Some(v) => Err(v),
            None => Ok(Self { kern, file: fp }),
        }
    }
}

impl<K: Kernel> Drop for OwnedFile<K> {
    fn drop(&mut self) {
        if unsafe { (*self.file).refcnt().fetch_sub(1, Ordering::Release) } != 1 {
            return;
        }

        fence(Ordering::Acquire);

        unsafe { self.kern.fdrop(self.file, K::Pcpu::curthread()) };
    }
}