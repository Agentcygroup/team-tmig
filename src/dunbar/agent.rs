use serde::{Deserialize, Serialize};
pub const GRANULARITY: f32 = 0.0030;
pub const AGENTS_PER_SWARM: usize = 333;
pub const NEIGHBORS: usize = 7;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct FineAgent {
    pub byte: u8,
    pub tick: u64,
}

impl FineAgent {
    pub fn new(byte: u8) -> Self {
        Self { byte, tick: 0 }
    }
    pub fn step(&mut self, neighbors: &[u8; NEIGHBORS]) {
        let sum: u32 = neighbors.iter().map(|&n| n as u32).sum();
        let mean = (sum / NEIGHBORS as u32) as u8;
        let max = *neighbors.iter().max().unwrap_or(&0);
        let sep = (self.byte as i16 - max as i16) as i16;
        self.byte = (mean as i16 + sep / 2).clamp(0, 255) as u8;
        self.tick = self.tick.wrapping_add(1);
    }
}
