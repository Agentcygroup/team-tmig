use serde::{Deserialize, Serialize};

pub type Hash256 = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Purpose {
    Audit,
    Legal,
    Sale,
    SourceOffer,
    Internal,
}

impl Default for Purpose {
    fn default() -> Self {
        Self::Internal
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeletionPolicy {
    DeleteAfterTtl,
    TombstoneAfterTtl,
    Never,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Retention {
    pub ttl_seconds: u64,
    pub deletion_policy: DeletionPolicy,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Disclosure {
    pub recipient: String,
    pub purpose: Purpose,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Declaration {
    pub claim_id: String,
    pub subject: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub evidence_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provenance {
    pub artifact_hash: String,
    pub source_hash: String,
    pub transform: String,
}
