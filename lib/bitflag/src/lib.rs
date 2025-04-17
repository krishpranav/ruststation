#![no_std]

mod mask;

use core::ops::{BitOr, Not};

pub trait Type: From<Self::Raw> {
    type Raw: Raw;
}

pub trait Raw: BitOr<Output = Self> + Not<Output = Self> + Copy {}

impl Raw for u32 {}