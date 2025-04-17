use super::PhysMap;
use core::num::NonZero;

#[repr(C)]
pub struct Vm {
    pub vmm: usize,
    pub console: usize,
    pub host_page_size: NonZero<usize>,
    pub memory_map: [PhysMap; 64],
}

#[cfg(feature = "virt")]
#[repr(C)]
pub struct VmmMemory {
    pub shutdown: KernelExit
}

pub enum KernelExit {
    Success,
    Panic
}