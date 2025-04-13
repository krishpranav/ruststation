use bitflags::bitflags;
use core::ffi::c_int;

pub trait Malloc: Sized {}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct MallocFlags: c_int {
        const WAITOK = 0x2;
        const ZERO = 0x100;
    }
}