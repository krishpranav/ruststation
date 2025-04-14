use crate::thread::Thread;
use crate::ucred::Ucred;
use core::ffi::{c_char, c_int};
use rsf::Kernel;

#[repr(C)]
pub struct ComponentName {
    op: u64,
    flags: u64,
    td: *mut Thread,
    cred: *mut Ucred,
    lk: c_int,
    buf: *mut c_char,
    name: *mut c_char,
    len: isize,
    consume: isize,
}

impl rsf::namei::ComponentName<crate::Kernel> for ComponentName {
    unsafe fn new(k: crate::Kernel, op: u64, lk: c_int, buf: *mut c_char, td: *mut Thread) -> Self {
        use rsf::thread::Thread;

        Self {
            op,
            flags: 0,
            td,
            cred: unsafe { (*td).cred() },
            lk,
            buf,
            name: buf,
            len: unsafe { k.strlen(buf) as _ },
            consume: 0,
        }
    }
}