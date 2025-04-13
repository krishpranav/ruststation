use crate::Kernel;

pub trait Thread<K: Kernel>: Sized {
    fn cred(&self) -> *mut K::Ucred;

    fn ret(&self, i: usize) -> usize;
}