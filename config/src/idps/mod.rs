#[repr(C)]
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ConsoleId {
    magic: u16,
    company: CompanyId,
    product: ProductId,
    prodsub: u16,
    #[cfg_attr(feature = "serde", serde(with = "serde_bytes"))]
    serial: [u8; 8],
}

impl ConsoleId {
    pub fn new(company: CompanyId, product: ProductId, prodsub: u16, serial: [u8; 8]) -> Self {
        Self {
            magic: 0,
            company,
            product,
            prodsub,
            serial,
        }
    }
}

impl Default for ConsoleId {
    fn default() -> Self {
        Self::new(
            CompanyId::SONY,
            ProductId::USA,
            0x1200,
            [0x10, 0, 0, 0, 0, 0, 0, 0],
        )
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct CompanyId(u16);

impl CompanyId {
    pub const SONY: Self = Self(0x100);
}

#[repr(transparent)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ProductId(u16);

impl ProductId {
    pub const DEVKIT: Self = Self(0x8101);
    pub const TESTKIT: Self = Self(0x8201);
    pub const USA: Self = Self(0x8401);
}