pub struct Flock {
    pub members: Vec<u64>,
    pub size: usize,
}
impl Flock {
    pub fn new() -> Self { Self { members: Vec::new(), size: 0 } }
    pub fn add(&mut self, id: u64) { self.members.push(id); self.size += 1; }
}
impl Default for Flock { fn default() -> Self { Self::new() } }
