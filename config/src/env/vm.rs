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
    pub shutdown: KernelExit,
}

#[cfg(feature = "virt")]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
pub enum KernelExit {
    Success,
    Panic,
}

#[cfg(feature = "virt")]
#[repr(C)]
pub struct ConsoleMemory {
    pub msg_len: NonZero<usize>,
    pub msg_addr: usize,
    pub commit: ConsoleType,
}

#[cfg(feature = "virt")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
pub enum ConsoleType {
    Info,
    Warn,
    Error,
}