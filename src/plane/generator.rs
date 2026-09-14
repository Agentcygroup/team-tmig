pub struct Generator {
    pub alphabet: Vec<String>,
    pub seed: u64,
    pub counter: u64,
}

impl Generator {
    pub fn new(alphabet: Vec<String>, seed: u64) -> Self {
        Self {
            alphabet,
            seed,
            counter: 0,
        }
    }
    pub fn next_command(&mut self) -> Option<String> {
        if self.alphabet.is_empty() {
            return None;
        }
        self.counter = self.counter.wrapping_add(1);
        let idx =
            (self.counter.wrapping_mul(self.seed.wrapping_add(1)) as usize) % self.alphabet.len();
        Some(self.alphabet[idx].clone())
    }
}
