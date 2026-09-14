use crate::release::{Artifact, ReleaseIndex};

pub struct Consumer {
    pub index: ReleaseIndex,
    pub fetched: Vec<Artifact>,
}

impl Consumer {
    pub fn new(index: ReleaseIndex) -> Self {
        Self { index, fetched: Vec::new() }
    }
    pub fn fetch(&mut self, name: &str) -> Option<Artifact> {
        self.index.artifacts.get(name).cloned().map(|a| {
            self.fetched.push(a.clone());
            a
        })
    }
    pub fn fetched_count(&self) -> usize { self.fetched.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::release::{Artifact, ArtifactKind, ReleaseIndex};
    #[test] fn fetch_by_name() {
        let mut idx = ReleaseIndex::new("1.0.0");
        idx.add(Artifact::new("bin", ArtifactKind::Binary, b"x"));
        let mut c = Consumer::new(idx);
        assert!(c.fetch("bin").is_some());
        assert_eq!(c.fetched_count(), 1);
    }
    #[test] fn missing_returns_none() {
        let idx = ReleaseIndex::new("1.0.0");
        let mut c = Consumer::new(idx);
        assert!(c.fetch("nope").is_none());
    }
}
