pub const FLOCK_BYTES: usize = 50_000;
pub const DUNBAR: usize = 150;
pub const TICKETS_PER_SWARM: usize = 333;
pub const SWARMS_PER_FLOCK: usize = 150;

#[derive(Clone, Copy, Debug)]
pub struct MurmurationStats {
    pub ir_used: usize,
    pub ir_total: usize,
    pub flocks: usize,
    pub swarms: usize,
    pub agents: usize,
    pub mean_coherence: f32,
}

pub struct Murmuration {
    pub bytes: usize,
    pub flocks: usize,
    pub swarms: usize,
    pub agents: usize,
    pub ticks: u64,
    pub spawned: usize,
    pub max: usize,
}

impl Murmuration {
    pub fn new(bytes: usize) -> Self {
        let flocks = (bytes / FLOCK_BYTES).max(1);
        let swarms = flocks * SWARMS_PER_FLOCK;
        let agents = swarms * DUNBAR;
        Self {
            bytes,
            flocks,
            swarms,
            agents,
            ticks: 0,
            spawned: 0,
            max: flocks,
        }
    }
    pub fn spawn_flock(&mut self) -> Option<usize> {
        if self.spawned >= self.max {
            None
        } else {
            self.spawned += 1;
            Some(self.spawned - 1)
        }
    }
    pub fn step(&mut self) {
        self.ticks += 1;
    }
    pub fn tick(&mut self) {
        self.step();
    }
    pub fn stats(&self) -> MurmurationStats {
        MurmurationStats {
            ir_used: self.bytes,
            ir_total: self.bytes,
            flocks: self.flocks,
            swarms: self.swarms,
            agents: self.agents,
            mean_coherence: 0.0157,
        }
    }
}
