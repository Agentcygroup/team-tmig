use crate::release::artifact::sha256;

#[derive(Clone, Debug)]
pub struct Provenance {
    pub artifact_hash: [u8; 32],
    pub source_hash: [u8; 32],
    pub recipe_hash: [u8; 32],
    pub builder: String,
    pub toolchain: String,
    pub timestamp: String,
    pub level: u8,
}

impl Provenance {
    pub fn new(
        artifact: &[u8],
        source: &[u8],
        recipe: &[u8],
        toolchain: &str,
        timestamp: &str,
    ) -> Self {
        Self {
            artifact_hash: sha256(artifact),
            source_hash: sha256(source),
            recipe_hash: sha256(recipe),
            builder: "tmig-build".into(),
            toolchain: toolchain.into(),
            timestamp: timestamp.into(),
            level: 3,
        }
    }
}
