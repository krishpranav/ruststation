use crate::Kernel;

#[repr(C)]
pub struct Pcpu {}

impl rsf::pcpu::Pcpu<Kernel> for Pcpu {}