use crate::Kernel;

pub trait Uio<K: Kernel>: Sized {
    unsafe fn write(iov: *mut IoVec, td: *mut K::Thread) -> Option<Self>;

    unsafe fn read(iov: *mut IoVec, off: usize, td: *mut K::Thread) -> Option<Self>;

    fn vec_max() -> usize {
        1024
    }

    fn io_max() -> usize {
        0x7fffffff
    }

    fn offset(&self) -> isize;

    fn remaining(&self) -> isize;
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