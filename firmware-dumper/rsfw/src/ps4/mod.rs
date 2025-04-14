#[cfg(feature = "read")]
pub use self::part::*;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "read")]
mod part;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum PartItem {
    End = 0,
    Directory = 1,
    File = 2,
}