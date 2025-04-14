!
mod elf;

#[no_std]

pub trait Kernel: Copy + Send + Sync + 'static {
}

pub trait KernelExt: Kernel {
}

impl<T: Kernel> KernelExt for T {
}