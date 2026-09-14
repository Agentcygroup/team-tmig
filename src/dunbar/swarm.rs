use super::agent::{FineAgent, AGENTS_PER_SWARM, NEIGHBORS};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Swarm {
    pub agents: Vec<FineAgent>,
    pub tick: u64,
    pub seed: u64,
}

impl Swarm {
    pub fn new(seed: u64) -> Self {
        let mut agents = Vec::with_capacity(AGENTS_PER_SWARM);
        for i in 0..AGENTS_PER_SWARM {
            let h = seed
                .wrapping_mul(0x9E3779B97F4A7C15)
                .wrapping_add((i as u64).wrapping_mul(0xBF58476D1CE4E5B9));
            agents.push(FineAgent::new((h ^ (h >> 29) ^ (h >> 47)) as u8));
        }
        Self {
            agents,
            tick: 0,
            seed,
        }
    }
    pub fn step(&mut self) {
        let prev: Vec<u8> = self.agents.iter().map(|a| a.byte).collect();
        for i in 0..AGENTS_PER_SWARM {
            let mut n = [0u8; NEIGHBORS];
            for k in 0..NEIGHBORS {
                n[k] = prev[(i + k * 47) % AGENTS_PER_SWARM];
            }
            self.agents[i].step(&n);
        }
        self.tick = self.tick.wrapping_add(1);
    }
    pub fn coherence(&self) -> f32 {
        let sum: u64 = self.agents.iter().map(|a| a.byte as u64).sum();
        sum as f32 / AGENTS_PER_SWARM as f32 / 255.0
    }
}
