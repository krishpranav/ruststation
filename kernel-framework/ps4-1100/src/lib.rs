use rsf::{Function, MappedKernel, StaticMut, offset};

pub mod ucred;
pub mod thread;
pub mod socket;
mod file;
mod lock;
mod malloc;
mod mount;
mod namei;
mod pcpu;

#[derive(Clone, Copy, MappedKernel)]
pub struct Kernel(*const u8);

impl rsf::Kernel for Kernel {
}

unsafe impl Send for Kernel {}
unsafe impl Sync for Kernel {}