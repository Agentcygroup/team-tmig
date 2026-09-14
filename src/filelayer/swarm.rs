use crate::filelayer::agent::LiveFileAgent;

pub struct FileSwarm {
    pub agents: Vec<LiveFileAgent>,
    pub ticks: u64,
}

impl FileSwarm {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            ticks: 0,
        }
    }
    pub fn add(&mut self, a: LiveFileAgent) {
        self.agents.push(a);
    }
    pub fn step(&mut self) {
        self.ticks += 1;
        for a in &mut self.agents {
            a.tick();
        }
    }
    pub fn tick(&mut self) {
        self.step();
    }
    pub fn stats(&self) -> (usize, f32) {
        let n = self.agents.len();
        let c = if n == 0 {
            0.0
        } else {
            self.agents.iter().map(|a| a.coherence).sum::<f32>() / n as f32
        };
        (n, c)
    }
}
impl Default for FileSwarm {
    fn default() -> Self {
        Self::new()
    }
}
