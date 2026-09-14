use std::collections::BTreeMap;

pub struct CacheEngine {
    pub store: BTreeMap<String, Vec<u8>>,
    pub hits: u64,
    pub misses: u64,
}

impl CacheEngine {
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
            hits: 0,
            misses: 0,
        }
    }
    pub fn get(&mut self, key: &str) -> Option<&[u8]> {
        if self.store.contains_key(key) {
            self.hits += 1;
        } else {
            self.misses += 1;
        }
        self.store.get(key).map(Vec::as_slice)
    }
    pub fn put(&mut self, key: &str, value: Vec<u8>) {
        self.store.insert(key.to_string(), value);
    }
}
impl Default for CacheEngine {
    fn default() -> Self {
        Self::new()
    }
}
