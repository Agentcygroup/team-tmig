use crate::payload::registry::Registry;
use crate::payload::instantiate::instantiate;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct PackManifest {
    pub name: String,
    pub entries: BTreeMap<String, [u8; 32]>,
}

#[derive(Clone, Debug)]
pub struct Pack {
    pub manifest: PackManifest,
    pub blobs: BTreeMap<String, Vec<u8>>,
}

impl Pack {
    pub fn from_registry(r: &Registry, seed: &[u8]) -> Self {
        let mut blobs = BTreeMap::new();
        let mut entries = BTreeMap::new();
        for s in &r.specs {
            let b = instantiate(s, seed);
            let h = crate::release::artifact::sha256(&b);
            blobs.insert(s.name.clone(), b);
            entries.insert(s.name.clone(), h);
        }
        Self {
            manifest: PackManifest { name: "omega-pack".into(), entries },
            blobs,
        }
    }
    pub fn len(&self) -> usize { self.blobs.len() }
    pub fn is_empty(&self) -> bool { self.blobs.is_empty() }
    pub fn total_bytes(&self) -> usize { self.blobs.values().map(|b| b.len()).sum() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn pack_from_registry() {
        let r = Registry::omega();
        let p = Pack::from_registry(&r, b"seed");
        assert_eq!(p.len(), r.len());
        assert!(p.total_bytes() > 0);
    }
}
