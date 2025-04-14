use crate::Kernel;

pub trait Uio<K: Kernel>: Sized {
    unsafe fn new(
        td: *mut K::Thread,
        op: UioRw,
        seg: UioSeg,
        iov: *mut IoVec,
        len: usize,
    ) -> Option<Self>;

    fn vec_max() -> usize {
        1024
    }

    fn io_max() -> usize {
        0x7fffffff
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UioSeg {
    User,
    Kernel,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UioRw {
    Read,
    Write,
}

#[repr(C)]
pub struct IoVec {
    pub ptr: *mut u8,
    pub len: usize,
}