use crate::swarm::mode::Mode;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub struct Constraints {
    pub max_agents: usize,
    pub max_ticks: u64,
    pub fault_threshold: u32,
}

impl Constraints {
    pub fn for_mode(_m: &Mode) -> Self {
        Self {
            max_agents: 100_000,
            max_ticks: 10_000,
            fault_threshold: 10,
        }
    }
    pub fn for_mode_str(s: &str) -> Self {
        match Mode::from_str(s) {
            Ok(m) => Self::for_mode(&m),
            Err(_) => Self::for_mode(&Mode::Adaptive),
        }
    }
}
