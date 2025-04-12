use crate::Kernel;
use core::ffi::c_int;

pub trait VnodeOp: Sized {}

pub trait VopUnlock: Sized {}

pub trait VopRead<K: Kernel>: Sized {
    unsafe fn new(
        k: K,
        vp: *mut K::Vnode,
        uio: *mut K::Uio,
        flags: c_int,
        cred: *mut K::Ucred,
    ) -> Self;
}

pub trait VopReadDir<K: Kernel>: Sized {
    unsafe fn new(
        k: K,
        vp: *mut K::Vnode,
        uio: *mut K::Uio,
        cred: *mut K::Ucred,
        eof: *mut c_int,
        ncookies: *mut c_int,
        cookies: *mut *mut u64,
    ) -> Self;
}

pub trait VopLookup<K: Kernel>: Sized {
    unsafe fn new(
        k: K,
        vp: *mut K::Vnode,
        out: *mut *mut K::Vnode,
        cn: *mut K::ComponentName,
    ) -> Self;
}