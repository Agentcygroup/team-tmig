use crate::content::index::ContentIndex;
use crate::content::registry::{all_kinds, Content, ContentKind};

pub fn derive_all(source: &str, seed: &[u8]) -> ContentIndex {
    let mut idx = ContentIndex::new();
    for kind in all_kinds() {
        let name = format!("{}-{:?}.derived", source, kind);
        let mut h = sha2::Sha256::new();
        use sha2::Digest;
        h.update(source.as_bytes());
        h.update(seed);
        h.update(format!("{:?}", kind).as_bytes());
        let d = h.finalize();
        let c = Content::new(&name, kind, &d, source);
        idx.add(c);
    }
    idx
}
