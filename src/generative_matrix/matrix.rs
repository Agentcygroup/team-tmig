use crate::generative_matrix::ALL_FEATURES;
use std::collections::BTreeMap;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Framework { LambdaCalculus, TypeTheory, CategoryTheory, ProcessCalculus, OntologyLanguage, FormalMethod, CapabilitySystem, AgentFramework, OurCrate }
pub const ALL_FRAMEWORKS: &[Framework] = &[Framework::LambdaCalculus, Framework::TypeTheory, Framework::CategoryTheory, Framework::ProcessCalculus, Framework::OntologyLanguage, Framework::FormalMethod, Framework::CapabilitySystem, Framework::AgentFramework, Framework::OurCrate];
pub struct GenMatrix { pub cells: BTreeMap<(Framework, crate::generative_matrix::Feature), bool> }
impl GenMatrix {
    pub fn new() -> Self { Self { cells: BTreeMap::new() } }
    pub fn set(&mut self, f: Framework, feat: crate::generative_matrix::Feature, v: bool) { self.cells.insert((f, feat), v); }
    pub fn has(&self, f: Framework, feat: crate::generative_matrix::Feature) -> bool { *self.cells.get(&(f, feat)).unwrap_or(&false) }
    pub fn novelty_count(&self) -> usize {
        ALL_FEATURES.iter().filter(|&&feat| !ALL_FRAMEWORKS[..8].iter().any(|&f| self.has(f, feat)) && self.has(Framework::OurCrate, feat)).count()
    }
}
impl Default for GenMatrix { fn default() -> Self { Self::new() } }
