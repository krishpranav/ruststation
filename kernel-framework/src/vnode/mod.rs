pub use self::dirent::*;
pub use self::op::*;
use crate::Kernel;
use core::ffi::c_int;

mod dirent;
mod op;

pub trait Vnode<K: Kernel>: Sized {
    fn ty(&self) -> c_int;

    fn ops(&self) -> *mut K::VopVector;
}

pub trait VopVector: Sized {}