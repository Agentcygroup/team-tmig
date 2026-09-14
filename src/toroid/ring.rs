#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RingIndex(pub u32);
pub struct Ring {
    pub m: u32,
}
impl Ring {
    pub fn new(m: u32) -> Option<Self> {
        if m == 0 {
            None
        } else {
            Some(Self { m })
        }
    }
    pub fn wrap(&self, x: i64) -> RingIndex {
        RingIndex((((x % self.m as i64) + self.m as i64) % self.m as i64) as u32)
    }
}
