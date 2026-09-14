use crate::release::Artifact;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct ReleaseIndex {
    pub version: String,
    pub artifacts: BTreeMap<String, Artifact>,
}

impl ReleaseIndex {
    pub fn new(v: &str) -> Self {
        Self {
            version: v.into(),
            artifacts: BTreeMap::new(),
        }
    }
    pub fn add(&mut self, a: Artifact) {
        self.artifacts.insert(a.name.clone(), a);
    }
    pub fn len(&self) -> usize {
        self.artifacts.len()
    }
    pub fn is_empty(&self) -> bool {
        self.artifacts.is_empty()
    }
    pub fn total_size(&self) -> u64 {
        self.artifacts.values().map(|a| a.size).sum()
    }
    pub fn to_json(&self) -> String {
        format!(
            "{{\"version\":\"{}\",\"count\":{}}}",
            self.version,
            self.artifacts.len()
        )
    }
}
