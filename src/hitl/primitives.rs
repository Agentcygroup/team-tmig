use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Declaration {
    pub claim_id: String,
    pub subject: String,
    pub evidence_path: String,
    pub evidence_hash: String,
    pub ts: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Consent {
    pub subject_id: String,
    pub signature: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Retention {
    pub ttl_seconds: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Disclosure {
    pub recipient: String,
    pub purpose: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Provenance {
    pub artifact_hash: String,
    pub source_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Revocation {
    pub event_hash: String,
    pub reason: String,
}
