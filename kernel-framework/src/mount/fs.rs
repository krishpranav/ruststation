use crate::Kernel;
use core::ffi::{c_char, c_int};
use core::num::NonZero;

pub trait Filesystem: Sized {
    fn name(&self) -> *const c_char;
}

pub trait FsOps<K: Kernel>: Sized {
    unsafe fn root(&self, mp: *mut K::Mount, flags: c_int)
                   -> Result<*mut K::Vnode, NonZero<c_int>>;
}