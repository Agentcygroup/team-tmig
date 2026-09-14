pub struct SimdEngine {
    pub lane_width: usize,
}

impl SimdEngine {
    pub fn new() -> Self {
        Self { lane_width: 16 }
    }
    pub fn xor_blocks(&self, a: &[u8], b: &[u8]) -> Vec<u8> {
        a.iter().zip(b).map(|(x, y)| x ^ y).collect()
    }
}
impl Default for SimdEngine {
    fn default() -> Self {
        Self::new()
    }
}
