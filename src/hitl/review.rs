//! Weekly / monthly pattern review. Aggregate over many actions.

use crate::hitl::ledger::{HitlEvent, HitlLedger};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct PatternReport {
    pub window_days: u32,
    pub proposed: usize,
    pub approved: usize,
    pub executed: usize,
    pub verified: usize,
    pub failed: usize,
    pub by_kind: BTreeMap<String, usize>,
    pub mean_risk: f32,
    pub notes: Vec<String>,
}

pub fn pattern_report(ledger: &HitlLedger, window_days: u32) -> PatternReport {
    let cutoff = chrono::Utc::now() - chrono::Duration::days(window_days as i64);
    let mut proposed = 0;
    let mut approved = 0;
    let mut executed = 0;
    let mut verified = 0;
    let mut failed = 0;
    let mut risk_sum = 0.0f32;
    let mut risk_n = 0;
    let mut by_kind: BTreeMap<String, usize> = BTreeMap::new();

    for e in ledger.events() {
        match e {
            HitlEvent::Proposed { ts, proposal, .. } if *ts >= cutoff => {
                proposed += 1;
                let kind = match &proposal.action {
                    crate::hitl::proposal::Action::ReadFile { .. } => "read",
                    crate::hitl::proposal::Action::WriteFile { .. } => "write",
                    crate::hitl::proposal::Action::RunCommand { .. } => "run",
                    crate::hitl::proposal::Action::Transfer { .. } => "transfer",
                    crate::hitl::proposal::Action::Deploy { .. } => "deploy",
                    crate::hitl::proposal::Action::Sign { .. } => "sign",
                    crate::hitl::proposal::Action::Custom { .. } => "custom",
                };
                *by_kind.entry(kind.into()).or_insert(0) += 1;
                risk_sum += proposal.risk.score();
                risk_n += 1;
            }
            HitlEvent::Approved { ts, .. } if *ts >= cutoff => approved += 1,
            HitlEvent::Executed { ts, execution, .. } if *ts >= cutoff => {
                executed += 1;
                if matches!(
                    execution.outcome,
                    crate::hitl::executor::Outcome::Failure { .. }
                ) {
                    failed += 1;
                }
            }
            HitlEvent::Verified { ts, .. } if *ts >= cutoff => verified += 1,
            _ => {}
        }
    }

    let mean_risk = if risk_n == 0 {
        0.0
    } else {
        risk_sum / risk_n as f32
    };

    let mut notes = Vec::new();
    if executed > proposed * 2 {
        notes.push("execution count exceeds proposals — likely a policy loop".into());
    }
    if failed as f32 / executed.max(1) as f32 > 0.1 {
        notes.push(format!(
            "failure rate {}% exceeds 10%",
            (failed as f32 / executed.max(1) as f32 * 100.0) as i32
        ));
    }

    PatternReport {
        window_days,
        proposed,
        approved,
        executed,
        verified,
        failed,
        by_kind,
        mean_risk,
        notes,
    }
}
