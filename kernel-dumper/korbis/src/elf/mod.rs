use core::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgramType(u32);

impl ProgramType {
    pub const PT_NULL: ProgramType = ProgramType(0x0);

    pub fn new(v: u32) -> Self {
        Self(v)
    }
}

impl Display for ProgramType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::PT_NULL => f.write_str("PT_NULL")

            t => write!(f, "{:#010x}", t.0)
        }
    }
}