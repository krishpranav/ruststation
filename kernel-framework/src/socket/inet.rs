use super::{AF_INET, SockAddr};
use core::mem::{size_of, transmute};

pub const INADDR_ANY: u32 = 0;

#[repr(C)]
pub struct SockAddrIn {
    pub sin_len: u8,
    pub sin_family: u8,
    pub sin_port: u16,
    pub sin_addr: InAddr,
    pub sin_zero: [u8; 8],
}

impl SockAddrIn {
    pub fn new(addr: InAddr, port: u16) -> Self {
        Self {
            sin_len: size_of::<Self>() as _,
            sin_family: AF_INET as _,
            sin_port: port.to_be(),
            sin_addr: addr,
            sin_zero: [0; 8],
        }
    }
}

impl AsRef<SockAddr> for SockAddrIn {
    fn as_ref(&self) -> &SockAddr {
        unsafe { transmute(self) }
    }
}

impl AsMut<SockAddr> for SockAddrIn {
    fn as_mut(&mut self) -> &mut SockAddr {
        unsafe { transmute(self) }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InAddr {
    pub s_addr: u32,
}

impl InAddr {
    pub const ANY: Self = Self { s_addr: 0 };
}

impl From<u32> for InAddr {
    fn from(value: u32) -> Self {
        Self { s_addr: value }
    }
}