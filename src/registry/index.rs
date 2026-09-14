use std::collections::BTreeMap;

#[derive(Default)]
pub struct IndexRegistry {
    by_name: BTreeMap<String, [u8; 32]>,
}

impl IndexRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn bind(&mut self, name: &str, hash: [u8; 32]) {
        self.by_name.insert(name.to_string(), hash);
    }
    pub fn resolve(&self, name: &str) -> Option<[u8; 32]> {
        self.by_name.get(name).copied()
    }
    pub fn count(&self) -> usize {
        self.by_name.len()
    }
}
