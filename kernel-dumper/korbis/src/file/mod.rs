use crate::thread::Thread;
use crate::Kernel;
use core::sync::atomic::{fence, AtomicU32, Ordering};

pub trait File: Sized {
    fn refcnt(&self) -> AtomicU32;
}

pub struct OwnedFile<K: Kernel> {
    kernel: K,
    file: *mut K::File,
}