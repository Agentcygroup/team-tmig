#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArtifactKind {
    Binary,
    Library,
    Doc,
    Schema,
    Dataset,
    Source,
}
#[derive(Clone, Debug)]
pub struct Artifact {
    pub name: String,
    pub kind: ArtifactKind,
    pub hash: [u8; 32],
    pub size: u64,
}
impl Artifact {
    pub fn new(n: &str, k: ArtifactKind, b: &[u8]) -> Self {
        Self {
            name: n.into(),
            kind: k,
            hash: sha256(b),
            size: b.len() as u64,
        }
    }
}
pub fn sha256(b: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    Sha256::digest(b).into()
}
