pub mod admit;
pub mod attest;
pub mod consequence;
pub mod scope;
pub mod thesis;

pub use admit::{admit, ActionProof, AdmitError};
pub use attest::{route_for, Attestation, DunbarLayer, FlockPolarity};
pub use consequence::Consequence;
pub use scope::Scope;
pub use thesis::Thesis;

pub type Hash256 = [u8; 32];

pub fn hash_bytes(b: &[u8]) -> Hash256 {
    use sha2::{Digest, Sha256};
    Sha256::digest(b).into()
}

pub fn hex(h: &Hash256) -> String {
    h.iter().map(|x| format!("{:02x}", x)).collect()
}

pub struct ControlPlane {
    pub thesis: Thesis,
    pub scope: Scope,
    pub doctrine_hash: Hash256,
    pub catalog_hash: Hash256,
    pub skills_hash: Hash256,
}

impl ControlPlane {
    pub fn derive(doctrine: &str, catalog: &str, skills: &str) -> Self {
        Self {
            thesis: Thesis::from_doctrine(doctrine),
            scope: Scope::from_doctrine(doctrine),
            doctrine_hash: hash_bytes(doctrine.as_bytes()),
            catalog_hash: hash_bytes(catalog.as_bytes()),
            skills_hash: hash_bytes(skills.as_bytes()),
        }
    }
    pub fn from_disk() -> Self {
        Self::derive("content-addressed human-authorized reversible", "", "")
    }
    pub fn hash(&self) -> Hash256 {
        let mut b = Vec::new();
        b.extend_from_slice(&self.doctrine_hash);
        b.extend_from_slice(&self.catalog_hash);
        b.extend_from_slice(&self.skills_hash);
        hash_bytes(&b)
    }
    pub fn hex_hash(&self) -> String {
        hex(&self.hash())
    }
    pub fn handle(&self, p: &ActionProof) -> Result<Attestation, AdmitError> {
        admit(&self.thesis, &self.scope, p)?;
        Ok(Attestation::from(p, self.doctrine_hash))
    }
}
