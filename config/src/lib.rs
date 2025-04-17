#![no_std]

pub use self::env::*;
pub use self::idps::*;

use core::num::NonZero;

mod env;
mod idps;

#[repr(C)]
pub enum BootEnv {
    Vm(Vm),
}

#[repr(C)]
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Config {
    pub max_cpu: NonZero<usize>,
    pub idps: ConsoleId,
    #[cfg_attr(feature = "serde", serde(with = "serde_bytes"))]
    pub env_vars: [u8; 132096],
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_cpu: NonZero::new(1).unwrap(),
            idps: ConsoleId::default(),
            env_vars: [0; 132096],
        }
    }
}