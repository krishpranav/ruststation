use crate::Kernel;
use crate::thread::Thread;
use core::ffi::{c_int, c_short, c_ushort};
use core::num::NonZero;

pub const AF_INET: c_int = 2;

#[repr(C)]
pub struct SockAddr {
    pub sa_len: u8,
    pub sa_family: u8,
    pub sa_data: [u8; 14],
}