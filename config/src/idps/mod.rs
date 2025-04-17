#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ConsoleId {
    magic: u16,
}

impl ConsoleId {
}

impl Default for ConsoleId {
    fn default() -> Self {
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct CompanyId(u16);

impl CompanyId {
    pub const SONY: Self = Self(0x100);
}