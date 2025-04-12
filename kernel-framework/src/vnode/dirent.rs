use core::ffi::c_char;

#[repr(C)]
pub struct DirEntry<const L: usize> {
    pub id: u32,
    pub len: u16,
    pub name: [c_char; L],
}