use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub type Hash = String;

pub struct ObjectStore { root: PathBuf }

impl ObjectStore {
    pub fn open(root: impl Into<PathBuf>) -> std::io::Result<Self> {
        let root = root.into();
        std::fs::create_dir_all(root.join("objects"))?;
        Ok(Self { root })
    }
    pub fn hash(bytes: &[u8]) -> Hash {
        let mut h = Sha256::new();
        h.update(bytes);
        h.finalize().iter().map(|b| format!("{:02x}", b)).collect()
    }
    fn path(&self, hash: &Hash) -> PathBuf {
        self.root.join("objects").join(&hash[..2]).join(&hash[2..])
    }
    pub fn put(&self, bytes: &[u8]) -> std::io::Result<Hash> {
        let hash = Self::hash(bytes);
        let p = self.path(&hash);
        if !p.exists() {
            std::fs::create_dir_all(p.parent().unwrap())?;
            std::fs::write(&p, bytes)?;
        }
        Ok(hash)
    }
    pub fn get(&self, hash: &Hash) -> std::io::Result<Vec<u8>> {
        std::fs::read(self.path(hash))
    }
}
