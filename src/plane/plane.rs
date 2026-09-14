use crate::plane::command::Command;
use crate::plane::generator::Generator;
use crate::plane::terminal::Terminal;

#[derive(Clone, Debug)]
pub struct PlaneStats {
    pub terminals: usize,
    pub ticks: u64,
    pub total_commands: usize,
}

pub struct Plane {
    pub id: u64,
    pub terminals: Vec<Terminal>,
    pub ticks: u64,
    pub generator: Generator,
}

impl Plane {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            terminals: Vec::new(),
            ticks: 0,
            generator: Generator::new(Vec::new(), 0),
        }
    }
    pub fn new_full(terminals: usize, alphabet: Vec<String>, seed: u64) -> Self {
        let mut p = Self::new(seed);
        p.generator = Generator::new(alphabet, seed);
        for i in 0..terminals {
            p.terminals.push(Terminal::new(i as u64));
        }
        p
    }
    pub fn push(&mut self, t: Terminal) {
        self.terminals.push(t);
    }
    pub fn run(&mut self, ticks: u64) {
        for _ in 0..ticks {
            self.step();
        }
    }
    pub fn step(&mut self) {
        self.ticks += 1;
        for t in &mut self.terminals {
            if let Some(cmd) = self.generator.next_command() {
                t.push(Command::new(cmd, ""));
            }
            t.step();
        }
    }
    pub fn stats(&self) -> PlaneStats {
        PlaneStats {
            terminals: self.terminals.len(),
            ticks: self.ticks,
            total_commands: self.terminals.iter().map(|t| t.commands.len()).sum(),
        }
    }
    pub fn count(&self) -> usize {
        self.terminals.len()
    }
    pub fn total_commands(&self) -> usize {
        self.terminals.iter().map(|t| t.commands.len()).sum()
    }
}
