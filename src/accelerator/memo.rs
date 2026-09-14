use std::collections::BTreeMap;

pub struct MemoEngine<K: Ord + Clone, V: Clone> {
    pub store: BTreeMap<K, V>,
}

impl<K: Ord + Clone, V: Clone> MemoEngine<K, V> {
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
        }
    }
    pub fn memoized<F>(&mut self, key: K, f: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(v) = self.store.get(&key) {
            return v.clone();
        }
        let v = f();
        self.store.insert(key, v.clone());
        v
    }
}
impl<K: Ord + Clone, V: Clone> Default for MemoEngine<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
