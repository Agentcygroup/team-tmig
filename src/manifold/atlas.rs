use crate::manifold::chart::Chart;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Atlas {
    pub charts: BTreeMap<String, Chart>,
}

impl Atlas {
    pub fn new() -> Self { Self::default() }
    pub fn insert(&mut self, c: Chart) { self.charts.insert(c.name.clone(), c); }
    pub fn len(&self) -> usize { self.charts.len() }
    pub fn is_empty(&self) -> bool { self.charts.is_empty() }
    pub fn torus() -> Self {
        let mut a = Self::new();
        a.insert(Chart::torus_standard("main"));
        a.insert(Chart::torus_standard("alternate"));
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn torus_atlas_has_two_charts() {
        assert_eq!(Atlas::torus().len(), 2);
    }
}
