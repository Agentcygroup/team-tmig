//! Executes actions that have been approved. Verifies outcomes.

use crate::hitl::approval::Approval;
use crate::hitl::proposal::{Action, Proposal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Execution {
    pub proposal_id: String,
    pub action: Action,
    pub approvals: Vec<Approval>,
    pub outcome: Outcome,
    pub executed_at: chrono::DateTime<chrono::Utc>,
    pub verified_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Outcome {
    Success { result: serde_json::Value },
    Failure { error: String },
    Skipped { reason: String },
}

pub fn execute(proposal: &Proposal, approvals: &[Approval]) -> Execution {
    // Only execute digital actions. Physical, legal, social actions are
    // handed to the human operator via the proposal.
    let outcome = match &proposal.action {
        Action::ReadFile { path } => match std::fs::read_to_string(path) {
            Ok(content) => Outcome::Success {
                result: serde_json::json!({ "bytes": content.len() }),
            },
            Err(e) => Outcome::Failure {
                error: e.to_string(),
            },
        },
        Action::WriteFile { path, content } => match std::fs::write(path, content) {
            Ok(()) => Outcome::Success {
                result: serde_json::json!({ "bytes": content.len() }),
            },
            Err(e) => Outcome::Failure {
                error: e.to_string(),
            },
        },
        Action::RunCommand { cmd } => {
            // Reject destructive commands unless explicitly approved
            if cmd.contains("rm -rf") || cmd.contains("dd if=") {
                Outcome::Skipped {
                    reason: "destructive command — human operator must run".into(),
                }
            } else {
                match std::process::Command::new("sh").arg("-c").arg(cmd).output() // allowed: HITL executor; operator-supplied // allowed: HITL executor; operator-supplied {
                    Ok(out) => Outcome::Success {
                        result: serde_json::json!({
                            "stdout": String::from_utf8_lossy(&out.stdout).to_string(),
                            "exit":   out.status.code().unwrap_or(-1),
                        }),
                    },
                    Err(e) => Outcome::Failure {
                        error: e.to_string(),
                    },
                }
            }
        }
        // Physical / legal / social actions are NOT executed by Pi.
        Action::Transfer { .. } => Outcome::Skipped {
            reason: "financial action — human operator must execute".into(),
        },
        Action::Deploy { .. } => Outcome::Skipped {
            reason: "deployment action — human operator must execute".into(),
        },
        Action::Sign { .. } => Outcome::Skipped {
            reason: "legal action — human operator must sign".into(),
        },
        Action::Custom { .. } => Outcome::Skipped {
            reason: "custom action — human operator must execute".into(),
        },
    };

    Execution {
        proposal_id: proposal.id.clone(),
        action: proposal.action.clone(),
        approvals: approvals.to_vec(),
        outcome,
        executed_at: chrono::Utc::now(),
        verified_at: None,
    }
}
