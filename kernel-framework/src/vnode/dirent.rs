use core::ffi::c_char;

#[repr(C)]
pub struct DirEnt<const L: usize> {
    pub id: u32,
    pub len: u16,
    pub ty: u8,
    pub name_len: u8,
    pub name: [c_char; L],
}