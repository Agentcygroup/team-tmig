use std::collections::BTreeMap;

pub fn hash(b: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(b)
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect()
}

pub struct AuditStore {
    pub objects: BTreeMap<String, Vec<u8>>,
    pub log: Vec<String>,
}

impl AuditStore {
    pub fn new() -> Self {
        Self {
            objects: BTreeMap::new(),
            log: Vec::new(),
        }
    }
    pub fn put(&mut self, b: &[u8]) -> String {
        let h = hash(b);
        self.objects.entry(h.clone()).or_insert_with(|| b.to_vec());
        h
    }
    pub fn record(&mut self, path: &str, before: &str, after: &str) {
        self.log.push(format!("{}\t{}\t{}", path, before, after));
    }
    pub fn len(&mut self) -> usize {
        self.objects.len()
    }
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
}

impl Default for AuditStore {
    fn default() -> Self {
        Self::new()
    }
}
