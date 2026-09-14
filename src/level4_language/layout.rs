#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    pub size: usize,
    pub align: usize,
}

pub const fn layout_of_u8() -> Layout { Layout { size: 1, align: 1 } }
pub const fn layout_of_u32() -> Layout { Layout { size: 4, align: 4 } }
pub const fn layout_of_u64() -> Layout { Layout { size: 8, align: 8 } }
pub const fn layout_of_256() -> Layout { Layout { size: 32, align: 8 } }
