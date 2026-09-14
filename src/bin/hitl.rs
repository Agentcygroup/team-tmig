use clap::{Parser, Subcommand};
use tmig::hitl::{
use tmig::controlplane::ControlPlane;
use tmig::hitl::cp_bridge;
    Consequence, HitlLevel, Proposal, Action, Alternative, Risk,
    Approval, execute, meets_level, HitlLedger, HitlEvent,
    pattern_report,
};
use tmig::roles::Role;
use sha2::{Sha256, Digest};

#[derive(Parser)]
#[command(name = "hitl", version = "0.1.0")]
struct Cli { #[command(subcommand)] cmd: Cmd }

#[derive(Subcommand)]
enum Cmd {
    /// Propose an action. Prints proposal id.
    Propose {
        #[arg(long)] kind: String,      // read | write | run | transfer | deploy | sign
        #[arg(long)] arg: String,       // path | command | target | document
        #[arg(long, default_value = "0.1")] probability: f32,
        #[arg(long, default_value = "0.1")] impact: f32,
        #[arg(long)] notes: Option<String>,
    },
    /// Approve a proposal as a role.
    Approve {
        proposal_id: String,
        #[arg(long)] role: String,       // architect | operator | auditor | adversary
        #[arg(long)] comment: Vec<String>,
    },
    /// Execute a proposal (only if approved at required level).
    Execute { proposal_id: String },
    /// Verify an execution.
    Verify { proposal_id: String, #[arg(long)] ok: bool, #[arg(long)] note: String },
    /// Pattern review over the last N days.
    Review { #[arg(long, default_value = "7")] days: u32 },
    /// List pending proposals.
    Pending,
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn make_roles() -> (Role, Role, Role, Role) {
    (
        Role::generate("architect"),
        Role::generate("operator"),
        Role::generate("auditor"),
        Role::generate("adversary"),
    )
}

fn classify(kind: &str) -> Consequence {
    match kind {
        "read"     => Consequence::None,
        "write"    => Consequence::Moderate,
        "run"      => Consequence::Low,
        "transfer" => Consequence::High,
        "deploy"   => Consequence::High,
        "sign"     => Consequence::Legal,
        _          => Consequence::Low,
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Propose { kind, arg, probability, impact, notes } => {
            let consequence = classify(&kind);
            let action = match kind.as_str() {
                "read"     => Action::ReadFile { path: arg.clone() },
                "write"    => Action::WriteFile { path: arg.clone(), content: String::new() },
                "run"      => Action::RunCommand { cmd: arg.clone() },
                "transfer" => Action::Transfer { to: arg.clone(), amount: 100 },
                "deploy"   => Action::Deploy { env: "prod".into(), version: arg.clone() },
                "sign"     => Action::Sign { document: arg.clone() },
                _          => Action::Custom { kind: kind.clone(), body: serde_json::json!({ "arg": arg }) },
            };
            let mut h = Sha256::new();
            h.update(kind.as_bytes());
            h.update(arg.as_bytes());
            h.update(chrono::Utc::now().to_rfc3339().as_bytes());
            let id: [u8; 32] = h.finalize().into();

            let proposal = Proposal {
                id: hex(&id),
                description: format!("{}: {}", kind, arg),
                consequence,
                action,
                alternatives: vec![
                    Alternative {
                        description: "Do nothing".into(),
                        action: Action::ReadFile { path: "/dev/null".into() },
                        tradeoff: "No effect, but also no progress".into(),
                    },
                ],
                risk: Risk { probability, impact, notes: notes.unwrap_or_default() },
                requires: vec![],
            };

            let mut ledger = HitlLedger::open()?;

        // ---- control plane admission ----
        {
            let plane = ControlPlane::from_disk();
            let approvers: Vec<String> = approvals.iter().map(|a| a.role.clone()).collect();
            let proof = cp_bridge::prove(&proposal, &approvers);
            match plane.handle(&proof) {
                Ok(att) => {
                    eprintln!("  cp-admit: ok  polarity={:?} layer={:?} consequence={:?}",
                        att.polarity, att.layer, att.consequence);
                }
                Err(e) => {
                    eprintln!("  cp-admit: REJECTED  {:?}", e);
                    std::process::exit(3);
                }
            }
        }
        // ---- end control plane admission ----
            let event_hash = ledger.append(HitlEvent::Proposed {
                ts: chrono::Utc::now(),
                proposal: proposal.clone(),
            })?;
            println!("proposal id:  {}", proposal.id);
            println!("consequence:  {:?}", consequence);
            println!("hitl level:   {:?}", consequence.hitl_level());
            println!("event hash:   {}", event_hash);
        }
        Cmd::Approve { proposal_id, role, comment } => {
            let roles = make_roles();
            let r = match role.as_str() {
                "architect" => roles.0,
                "operator"  => roles.1,
                "auditor"   => roles.2,
                "adversary" => roles.3,
                _ => anyhow::bail!("unknown role: {}", role),
            };
            let mut ledger = HitlLedger::open()?;
            // Find proposal, its level
            let proposal = ledger.proposals().into_iter().find(|p| p.id == proposal_id).cloned()
                .ok_or_else(|| anyhow::anyhow!("proposal not found"))?;
            let level = proposal.consequence.hitl_level();
            let approval = Approval::sign(&proposal_id, level, &r, comment.first().cloned())?;
            let event_hash = ledger.append(HitlEvent::Approved {
                ts: chrono::Utc::now(),
                approval: approval.clone(),
            })?;
            println!("approved by:  {}", approval.approver);
            println!("level:        {:?}", level);
            println!("signature:    {}...", &approval.signature[..16]);
            println!("event hash:   {}", event_hash);
        }
        Cmd::Execute { proposal_id } => {
            let mut ledger = HitlLedger::open()?;
            let proposal = ledger.proposals().into_iter().find(|p| p.id == proposal_id).cloned()
                .ok_or_else(|| anyhow::anyhow!("proposal not found"))?;
            let approvals: Vec<Approval> = ledger.approvals_for(&proposal_id)
                .into_iter().cloned().collect();
            let level = proposal.consequence.hitl_level();
            if !meets_level(level, &approvals) {
                anyhow::bail!("insufficient approvals for level {:?}", level);
            }
            let execution = execute(&proposal, &approvals);
            let event_hash = ledger.append(HitlEvent::Executed {
                ts: chrono::Utc::now(),
                execution: execution.clone(),
            })?;
            println!("outcome:      {:?}", execution.outcome);
            println!("event hash:   {}", event_hash);
        }
        Cmd::Verify { proposal_id, ok, note } => {
            let mut ledger = HitlLedger::open()?;
            let event_hash = ledger.append(HitlEvent::Verified {
                ts: chrono::Utc::now(),
                proposal_id: proposal_id.clone(),
                ok,
                note: note.clone(),
            })?;
            println!("verified:     {} ok={}", proposal_id, ok);
            println!("event hash:   {}", event_hash);
        }
        Cmd::Review { days } => {
            let ledger = HitlLedger::open()?;
            let report = pattern_report(&ledger, days);
            println!("pattern review (last {} days)", report.window_days);
            println!("  proposed:  {}", report.proposed);
            println!("  approved:  {}", report.approved);
            println!("  executed:  {}", report.executed);
            println!("  verified:  {}", report.verified);
            println!("  failed:    {}", report.failed);
            println!("  mean risk: {:.3}", report.mean_risk);
            println!("  by kind:");
            for (k, n) in &report.by_kind {
                println!("    {:<12} {}", k, n);
            }
            if !report.notes.is_empty() {
                println!("  notes:");
                for n in &report.notes {
                    println!("    - {}", n);
                }
            }
        }
        Cmd::Pending => {
            let ledger = HitlLedger::open()?;
            for p in ledger.pending() {
                println!("{}  {:?}  {}", p.id, p.consequence, p.description);
            }
        }
    }
    Ok(())
}
