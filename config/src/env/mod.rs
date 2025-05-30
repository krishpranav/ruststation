pub use self::vm::*;

mod vm;

#[repr(C)]
pub struct PhysMap {
    pub base: u64,
    pub len: u64,
    pub ty: MapType,
    pub attrs: u32,
}

/// Type of [PhysMap].
#[repr(u32)]
pub enum MapType {
    None = 0,
    Ram = 1,
    Reserved = 2,
}