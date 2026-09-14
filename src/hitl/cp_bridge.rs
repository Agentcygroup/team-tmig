//! Bridge between hitl::Proposal and controlplane::ActionProof.
//!
//! The bridge is the only place where a hitl consequence class is mapped
//! to the control plane's consequence class, and the only place where an
//! Action is reduced to a scope-checkable target string. Change either
//! mapping here and the plane's behavior changes; no other file needs to.

use crate::controlplane::{ActionProof, Consequence as CpConsequence};
use crate::hitl::consequence::Consequence as HitlConsequence;
use crate::hitl::proposal::{Action, Proposal};
use crate::ledger::{hash_bytes, hex};

pub fn target_of(action: &Action) -> String {
    match action {
        Action::ReadFile { path } => path.clone(),
        Action::WriteFile { path, .. } => path.clone(),
        Action::RunCommand { cmd } => cmd.clone(),
        Action::Transfer { to, .. } => format!("transfer:{}", to),
        Action::Deploy { env, .. } => format!("deploy:{}", env),
        Action::Sign { document } => document.clone(),
        Action::Custom { kind, .. } => format!("custom:{}", kind),
    }
}

pub fn consequence_to_cp(c: HitlConsequence) -> CpConsequence {
    match c {
        HitlConsequence::None => CpConsequence::None,
        HitlConsequence::Low => CpConsequence::Low,
        HitlConsequence::Moderate => CpConsequence::Moderate,
        HitlConsequence::High => CpConsequence::High,
        HitlConsequence::Legal => CpConsequence::Legal,
        HitlConsequence::Irreversible => CpConsequence::Irreversible,
    }
}

pub fn prove(proposal: &Proposal, approvers: &[String]) -> ActionProof {
    let target = target_of(&proposal.action);
    let consequence = consequence_to_cp(proposal.consequence);

    let canon = serde_json::json!({
        "id": proposal.id,
        "description": proposal.description,
        "target": target,
        "consequence": format!("{:?}", proposal.consequence),
        "approvers": approvers,
    });
    let content_hash = hash_bytes(canon.to_string().as_bytes());

    let attestation_id =
        hash_bytes(format!("attestation:{}:{}", proposal.id, hex(&content_hash)).as_bytes());

    ActionProof {
        content_hash,
        target,
        consequence: Some(consequence),
        approvals: approvers.to_vec(),
        reversible: matches!(
            proposal.consequence,
            HitlConsequence::None
                | HitlConsequence::Low
                | HitlConsequence::Moderate
                | HitlConsequence::High
        ),
        attestation_id,
        primitives: Vec::new(),
        required_agent: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hitl::proposal::{Alternative, Risk};

    fn mk(action: Action, c: HitlConsequence) -> Proposal {
        Proposal {
            id: "test-1".into(),
            description: "test".into(),
            consequence: c,
            action,
            alternatives: vec![Alternative {
                description: "do nothing".into(),
                action: Action::ReadFile {
                    path: "/dev/null".into(),
                },
                tradeoff: "no effect".into(),
            }],
            risk: Risk {
                probability: 0.1,
                impact: 0.2,
                notes: String::new(),
            },
            requires: vec![],
        }
    }

    #[test]
    fn target_of_read_is_path() {
        assert_eq!(
            target_of(&Action::ReadFile {
                path: "src/lib.rs".into()
            }),
            "src/lib.rs"
        );
    }

    #[test]
    fn target_of_transfer_is_namespaced() {
        assert_eq!(
            target_of(&Action::Transfer {
                to: "vendor-x".into(),
                amount: 100
            }),
            "transfer:vendor-x"
        );
    }

    #[test]
    fn consequence_maps_1to1() {
        for (h, cp) in [
            (HitlConsequence::None, CpConsequence::None),
            (HitlConsequence::Low, CpConsequence::Low),
            (HitlConsequence::Moderate, CpConsequence::Moderate),
            (HitlConsequence::High, CpConsequence::High),
            (HitlConsequence::Legal, CpConsequence::Legal),
            (HitlConsequence::Irreversible, CpConsequence::Irreversible),
        ] {
            assert_eq!(consequence_to_cp(h), cp);
        }
    }

    #[test]
    fn prove_is_stable() {
        let p = mk(
            Action::ReadFile {
                path: "src/lib.rs".into(),
            },
            HitlConsequence::Low,
        );
        let approvers = vec!["operator".to_string()];
        let a = prove(&p, &approvers);
        let b = prove(&p, &approvers);
        assert_eq!(a.content_hash, b.content_hash);
    }

    #[test]
    fn irreversible_is_not_reversible() {
        let p = mk(
            Action::Sign {
                document: "contract.pdf".into(),
            },
            HitlConsequence::Irreversible,
        );
        let proof = prove(&p, &[]);
        assert!(!proof.reversible);
    }
}
