//! A HITL fellow: a named, content-addressed role with a public key.

use crate::fellowship::unit::Layer;
use crate::skills::AgentKind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FellowKind {
    pub kind: AgentKind,
    pub layer: Layer,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fellow {
    pub id: [u8; 32],
    pub name: String,
    pub kind: AgentKind,
    pub pubkey: [u8; 32],
    pub layer: Layer,
}

impl Fellow {
    pub fn new(name: &str, kind: AgentKind, pubkey: [u8; 32], layer: Layer) -> Self {
        use sha2::{Digest, Sha256};
        let mut h = Sha256::new();
        h.update(name.as_bytes());
        h.update(pubkey);
        let mut id = [0u8; 32];
        id.copy_from_slice(&h.finalize());
        Self {
            id,
            name: name.into(),
            kind,
            pubkey,
            layer,
        }
    }

    pub fn kind(&self) -> FellowKind {
        FellowKind {
            kind: self.kind,
            layer: self.layer,
        }
    }
}

/// The HITL rule: a fellow cannot approve their own proposal.
pub fn hitl_ok(proposal_fellow: &Fellow, approver_fellow: &Fellow) -> bool {
    proposal_fellow.id != approver_fellow.id
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(name: &str, kind: AgentKind) -> Fellow {
        Fellow::new(name, kind, [0u8; 32], Layer::Ledger)
    }

    #[test]
    fn hitl_rejects_self_approval() {
        let a = f("a", AgentKind::Architect);
        assert!(!hitl_ok(&a, &a));
    }

    #[test]
    fn hitl_accepts_distinct() {
        let a = f("a", AgentKind::Architect);
        let b = f("b", AgentKind::Operator);
        assert!(hitl_ok(&a, &b));
    }

    #[test]
    fn id_is_content_addressed() {
        let a = f("a", AgentKind::Architect);
        let b = f("a", AgentKind::Architect);
        assert_eq!(a.id, b.id);
    }
}
