use crate::content::registry::{all_kinds, Content, ContentKind};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct ContentIndex {
    pub by_kind: BTreeMap<ContentKind, Vec<Content>>,
    pub total_bytes: u64,
}

impl ContentIndex {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&mut self, c: Content) {
        self.total_bytes += c.size;
        self.by_kind.entry(c.kind).or_default().push(c);
    }
    pub fn len(&self) -> usize {
        self.by_kind.values().map(Vec::len).sum()
    }
    pub fn is_empty(&self) -> bool {
        self.by_kind.is_empty()
    }
    pub fn kinds(&self) -> usize {
        self.by_kind.len()
    }
    pub fn count(&self, k: ContentKind) -> usize {
        self.by_kind.get(&k).map(Vec::len).unwrap_or(0)
    }
}
