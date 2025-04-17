#![no_std]

pub use self::mask::*;
use core::ops::{BitOr, Not};
mod mask;

pub trait Type: From<Self::Raw> {
    type Raw: Raw;
}

pub trait Raw: BitOr<Output = Self> + Not<Output = Self> + Copy {}

impl Raw for u32 {}