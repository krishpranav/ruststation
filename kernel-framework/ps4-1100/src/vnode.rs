use crate::namei::ComponentName;
use crate::ucred::Ucred;
use crate::uio::Uio;
use core::ffi::c_int;
use rsf::Kernel;

#[repr(C)]
pub struct Vnode {
    ty: c_int,
    pad1: [u8; 0xC],
    ops: *mut VopVector,
}

impl rsf::vnode::Vnode<crate::Kernel> for Vnode {
    fn ty(&self) -> c_int {
        self.ty
    }

    fn ops(&self) -> *mut VopVector {
        self.ops
    }
}

#[repr(C)]
pub struct VopVector {}

impl rsf::vnode::VopVector for VopVector {}

#[repr(C)]
pub struct VnodeOp {}

impl rsf::vnode::VnodeOp for VnodeOp {}

#[repr(C)]
pub struct VopUnlock {
    desc: *mut VnodeOp,
    vp: *mut Vnode,
    flags: c_int,
}

impl rsf::vnode::VopUnlock for VopUnlock {}

#[repr(C)]
pub struct VopRead {
    desc: *mut VnodeOp,
    vp: *mut Vnode,
    uio: *mut Uio,
    flags: c_int,
    cred: *mut Ucred,
}

impl rsf::vnode::VopRead<crate::Kernel> for VopRead {
    unsafe fn new(
        k: crate::Kernel,
        vp: *mut Vnode,
        uio: *mut Uio,
        flags: c_int,
        cred: *mut Ucred,
    ) -> Self {
        Self {
            desc: k.get(crate::Kernel::VOP_READ).as_mut_ptr(),
            vp,
            uio,
            flags,
            cred,
        }
    }
}

#[repr(C)]
pub struct VopReadDir {
    desc: *mut VnodeOp,
    vp: *mut Vnode,
    uio: *mut Uio,
    cred: *mut Ucred,
    eof: *mut c_int,
    ncookies: *mut c_int,
    cookies: *mut *mut u64,
}

impl rsf::vnode::VopReadDir<crate::Kernel> for VopReadDir {
    unsafe fn new(
        k: crate::Kernel,
        vp: *mut Vnode,
        uio: *mut Uio,
        cred: *mut Ucred,
        eof: *mut c_int,
        ncookies: *mut c_int,
        cookies: *mut *mut u64,
    ) -> Self {
        Self {
            desc: k.get(crate::Kernel::VOP_READDIR).as_mut_ptr(),
            vp,
            uio,
            cred,
            eof,
            ncookies,
            cookies,
        }
    }
}

#[repr(C)]
pub struct VopLookup {
    desc: *mut VnodeOp,
    vp: *mut Vnode,
    out: *mut *mut Vnode,
    cn: *mut ComponentName,
}

impl rsf::vnode::VopLookup<crate::Kernel> for VopLookup {
    unsafe fn new(
        k: crate::Kernel,
        vp: *mut Vnode,
        out: *mut *mut Vnode,
        cn: *mut ComponentName,
    ) -> Self {
        Self {
            desc: k.get(crate::Kernel::VOP_LOOKUP).as_mut_ptr(),
            vp,
            out,
            cn,
        }
    }
}