use std::collections::BTreeMap;

#[derive(Default)]
pub struct ContentRegistry {
    map: BTreeMap<[u8; 32], Vec<u8>>,
}

impl ContentRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn put(&mut self, hash: [u8; 32], bytes: Vec<u8>) -> bool {
        if let std::collections::btree_map::Entry::Vacant(e) = self.map.entry(hash) {
            e.insert(bytes);
            true
        } else {
            false
        }
    }
    pub fn get(&self, hash: &[u8; 32]) -> Option<&[u8]> {
        self.map.get(hash).map(Vec::as_slice)
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
