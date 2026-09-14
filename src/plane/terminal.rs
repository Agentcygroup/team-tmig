use crate::plane::command::Command;

#[derive(Clone, Debug)]
pub struct Terminal {
    pub id: u64,
    pub commands: Vec<Command>,
    pub ticks: u64,
}

impl Terminal {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            commands: Vec::new(),
            ticks: 0,
        }
    }
    pub fn push(&mut self, c: Command) {
        self.commands.push(c);
    }
    pub fn step(&mut self) {
        self.ticks += 1;
    }
    pub fn tick(&mut self) {
        self.step();
    }
}
