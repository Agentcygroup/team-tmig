use crate::controlplane::{Scope, Thesis};
use crate::hitl::consequence::Consequence;
use serde::{Deserialize, Serialize};

pub type Hash256 = [u8; 32];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActionProof {
    pub content_hash: Hash256,
    pub target: String,
    pub consequence: Option<Consequence>,
    pub approvals: Vec<String>,
    pub reversible: bool,
    pub attestation_id: Hash256,
    pub primitives: Vec<String>,
    pub required_agent: Option<String>,
    pub thesis_ok: bool,
    pub scope_ok: bool,
}

impl ActionProof {
    pub fn empty() -> Self {
        Self {
            content_hash: [0; 32],
            target: String::new(),
            consequence: None,
            approvals: Vec::new(),
            reversible: false,
            attestation_id: [0; 32],
            primitives: Vec::new(),
            required_agent: None,
            thesis_ok: false,
            scope_ok: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdmitError {
    Thesis(String),
    Scope(String),
}

pub fn admit(t: &Thesis, s: &Scope, p: &ActionProof) -> Result<(), AdmitError> {
    if t.content_addressed && p.content_hash == [0u8; 32] {
        return Err(AdmitError::Thesis("not content-addressed".into()));
    }
    if t.human_authorized && p.approvals.is_empty() {
        return Err(AdmitError::Thesis("not human-authorized".into()));
    }
    if t.reversible && !p.reversible {
        return Err(AdmitError::Thesis("not reversible".into()));
    }
    s.contains(&p.target).map_err(AdmitError::Scope)?;
    Ok(())
}
