use crate::Kernel;
use crate::thread::Thread;
use core::ffi::c_int;
use rsf::uio::{IoVec, UioRw, UioSeg};

#[repr(C)]
pub struct Uio {
    iov: *mut IoVec,
    len: c_int,
    off: isize,
    res: isize,
    seg: UioSeg,
    op: UioRw,
    td: *mut Thread,
}

impl rsf::uio::Uio<Kernel> for Uio {
    unsafe fn write(iov: *mut IoVec, td: *mut Thread) -> Option<Self> {
        let res = unsafe { (*iov).len };

        if res > Self::io_max() {
            return None;
        }

        Some(Self {
            iov,
            len: 1,
            off: -1,
            res: res.try_into().unwrap(),
            seg: UioSeg::Kernel,
            op: UioRw::Write,
            td,
        })
    }

    unsafe fn read(iov: *mut IoVec, off: usize, td: *mut Thread) -> Option<Self> {
        let res = unsafe { (*iov).len };

        if res > Self::io_max() {
            return None;
        }

        Some(Self {
            iov,
            len: 1,
            off: off.try_into().unwrap(),
            res: res.try_into().unwrap(),
            seg: UioSeg::Kernel,
            op: UioRw::Read,
            td,
        })
    }

    fn offset(&self) -> isize {
        self.off
    }

    fn remaining(&self) -> isize {
        self.res
    }
}