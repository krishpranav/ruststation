use crate::Kernel;
use core::ffi::{c_char, c_int};

pub trait ComponentName<K: Kernel>: Sized {
    unsafe fn new(k: K, op: u64, lk: c_int, buf: *mut c_char, td: *mut K::Thread) -> Self;
}