use crate::swarm::constraints::Constraints;
use crate::swarm::mode::Mode;

pub struct AthleticAgent {
    pub id: u64,
    pub mode: Mode,
    pub state: f32,
    pub tick: u64,
    pub faults: u32,
    pub thresh: u32,
    pub hist: Vec<f32>,
    pub grad: [f32; 8],
}

impl AthleticAgent {
    pub fn new(id: u64, mode: Mode) -> Self {
        Self {
            id,
            mode,
            state: 0.0,
            tick: 0,
            faults: 0,
            thresh: 5,
            hist: Vec::new(),
            grad: [0.0; 8],
        }
    }
    pub fn cycle(&mut self, n: &[u8; 8], c: &Constraints) {
        self.tick += 1;
        self.sense(n);
        self.interpret(n);
        self.decide();
        self.act();
        if self.tick > c.max_ticks {
            self.faults += 1;
        }
    }
    fn sense(&mut self, n: &[u8; 8]) {
        let s: f32 = n.iter().map(|&b| b as f32).sum::<f32>() / 8.0;
        self.state = s;
    }
    fn interpret(&mut self, n: &[u8; 8]) {
        for (i, &b) in n.iter().enumerate() {
            self.grad[i] = self.grad[i] * 0.99 + b as f32 * 0.01;
        }
    }
    fn decide(&mut self) {
        if self.state > self.thresh as f32 {
            self.state *= 0.5;
        }
    }
    fn act(&mut self) {
        self.hist.push(self.state);
        if self.hist.len() > 16 {
            self.hist.remove(0);
        }
    }
}
