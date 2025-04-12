use crate::Kernel;

pub trait Uio<K: Kernel>: Sized {
    unsafe fn write(iov: *mut IoVec, td: *mut K::Thread) -> Option<Self>;

    unsafe fn read(iov: *mut IoVec, off: usize, td: *mut K::Thread) -> Option<Self>;

    fn vec_max() -> usize {
        1024
    }
}

#[repr(C)]
pub enum UioSeg {

}