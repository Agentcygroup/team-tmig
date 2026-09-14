use crate::interdisciplinary::{ALL_DISCIPLINES, ALL_INDUSTRIES};
use std::collections::BTreeMap;
pub struct DisciplineMatrix { pub counts: BTreeMap<(usize, usize), u32> }
impl DisciplineMatrix {
    pub fn new() -> Self { Self { counts: BTreeMap::new() } }
    pub fn seed(&mut self, n: u32) {
        for (i, _) in ALL_INDUSTRIES.iter().enumerate() {
            for (j, _) in ALL_DISCIPLINES.iter().enumerate() { self.counts.insert((i, j), n); }
        }
    }
    pub fn cells(&self) -> usize { self.counts.len() }
    pub fn total(&self) -> u64 { self.counts.values().map(|&x| x as u64).sum() }
}
impl Default for DisciplineMatrix { fn default() -> Self { Self::new() } }
