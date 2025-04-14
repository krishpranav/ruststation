use crate::thread::Thread;
use crate::Kernel;
use core::sync::atomic::{fence, AtomicU32, Ordering};

pub trait File: Sized {
    fn refcnt(&self) -> &AtomicU32;
}

pub struct OwnedFile<K: Kernel> {
    kernel: K,
    file: *mut K::File,
}

impl<K: Kernel> OwnedFile<K> {
    pub unsafe fn new(kernel: K, file: *mut K::File) -> Self {
        Self { kernel, file }
    }
}

impl<K: Kernel> Drop for OwnedFile<K> {
    fn drop(&mut self) {
        if unsafe { (*self.file).refcnt().fetch_sub(1, Ordering::Release) } != 1 {
            return;
        }

        fence(Ordering::Acquire);

        unsafe { self.kernel.fdrop(self.file, K::Thread::current()) };
    }
}