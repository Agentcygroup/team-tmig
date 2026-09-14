#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dimension {
    D0,
    D1,
    D2,
    D3,
    Dn(usize),
}
impl Dimension {
    pub fn value(&self) -> usize {
        match self {
            Dimension::D0 => 0,
            Dimension::D1 => 1,
            Dimension::D2 => 2,
            Dimension::D3 => 3,
            Dimension::Dn(n) => *n,
        }
    }
}
