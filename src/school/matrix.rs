use crate::interdisciplinary::{ALL_DISCIPLINES, ALL_SCHOOLS};
use std::collections::BTreeMap;
pub struct SchoolMatrix { pub counts: BTreeMap<(usize, usize), u32> }
impl SchoolMatrix {
    pub fn new() -> Self { Self { counts: BTreeMap::new() } }
    pub fn seed(&mut self, n: u32) {
        for (i, _) in ALL_DISCIPLINES.iter().enumerate() {
            for (j, _) in ALL_SCHOOLS.iter().enumerate() { self.counts.insert((i, j), n); }
        }
    }
    pub fn cells(&self) -> usize { self.counts.len() }
    pub fn total(&self) -> u64 { self.counts.values().map(|&x| x as u64).sum() }
}
impl Default for SchoolMatrix { fn default() -> Self { Self::new() } }
