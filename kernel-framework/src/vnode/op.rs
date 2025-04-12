use crate::Kernel;
use core::ffi::c_int;

pub trait VnodeOp: Sized {}

pub trait VopUnlock: Sized {}

pub trait VopRead<K: Kernel>: Sized {
    unsafe fn new(
        k: K,
        vp: *mut K::Vnode,
        uio: *mut K::Uio,
        flags: c_int
    ) -> Self;
}

pub trait VopLookup<K: Kernel>: Sized {
    unsafe fn new(

    ) -> Self;
}