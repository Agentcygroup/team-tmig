use clap::{Parser, Subcommand};
use tmig::ledger::Intent;
use tmig::team::TeamTMIG;

#[derive(Parser)]
#[command(name = "tmig", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Init,
    SetIntent {
        #[arg(long)]
        construction: bool,
        #[arg(long)]
        coverage: bool,
        #[arg(long)]
        fan: bool,
        #[arg(long)]
        pattern: Option<String>,
        #[arg(long)]
        invariant: Option<String>,
        #[arg(long)]
        command: Option<String>,
        #[arg(long, default_value = ".")]
        root: String,
    },
    Execute {
        hash: String,
        #[arg(long)]
        kind: String,
        #[arg(long)]
        pattern: Option<String>,
        #[arg(long)]
        invariant: Option<String>,
        #[arg(long)]
        command: Option<String>,
        #[arg(long, default_value = ".")]
        root: String,
        #[arg(long, default_value = "1")]
        nodes: u64,
    },
    Audit,
    RedTeam,
    Rollback {
        hash: String,
    },
    Ledger {
        #[arg(long, default_value = "10")]
        last: usize,
    },
}

fn main() -> anyhow::Result<()> {
    match Cli::parse().cmd {
        Cmd::Init => {
            let t = TeamTMIG::init()?;
            println!("initialized. roles at ~/.tmig/roles/");
            println!(
                "  architect: {}",
                tmig::ledger::hex(&t.roles.architect.verifying)
            );
            println!(
                "  operator:  {}",
                tmig::ledger::hex(&t.roles.operator.verifying)
            );
            println!(
                "  auditor:   {}",
                tmig::ledger::hex(&t.roles.auditor.verifying)
            );
            println!(
                "  adversary: {}",
                tmig::ledger::hex(&t.roles.adversary.verifying)
            );
        }
        Cmd::SetIntent {
            construction,
            coverage,
            fan,
            pattern,
            invariant,
            command,
            root,
        } => {
            let mut t = TeamTMIG::init()?;
            let (kind, body) = if construction {
                (
                    "construction",
                    serde_json::json!({"kind":"construction","pattern":pattern,"root":root}),
                )
            } else if coverage {
                (
                    "coverage",
                    serde_json::json!({"kind":"coverage","invariant":invariant,"root":root}),
                )
            } else if fan {
                ("fan", serde_json::json!({"kind":"fan","command":command}))
            } else {
                anyhow::bail!("specify --construction, --coverage, or --fan");
            };
            let h = t.set_intent(Intent {
                kind: kind.into(),
                body,
                ts: chrono::Utc::now(),
            })?;
            println!("intent hash: {}", h);
        }
        Cmd::Execute {
            hash,
            kind,
            pattern,
            invariant,
            command,
            root,
            nodes,
        } => {
            let mut t = TeamTMIG::init()?;
            let body = match kind.as_str() {
                "construction" => {
                    serde_json::json!({"kind":"construction","pattern":pattern,"root":root})
                }
                "coverage" => {
                    serde_json::json!({"kind":"coverage","invariant":invariant,"root":root})
                }
                "fan" => serde_json::json!({"kind":"fan","command":command,"nodes":nodes}),
                _ => anyhow::bail!("unknown kind"),
            };
            println!("result hash: {}", t.execute(&hash, &body)?);
        }
        Cmd::Audit => {
            let t = TeamTMIG::init()?;
            let r = t.audit()?;
            println!("audit: {} entries, {} bad", r.total, r.bad);
        }
        Cmd::RedTeam => {
            let t = TeamTMIG::init()?;
            println!("red team: {} / 30 bounds held", t.red_team()?);
        }
        Cmd::Rollback { hash } => {
            let mut t = TeamTMIG::init()?;
            t.rollback(&hash)?;
            println!("rolled back");
        }
        Cmd::Ledger { last } => {
            let t = TeamTMIG::init()?;
            for e in t.ledger.entries().iter().rev().take(last) {
                println!(
                    "{}  intent={}  result={}",
                    &e.hash[..16],
                    &e.intent_hash[..16],
                    &e.result_hash[..16]
                );
            }
        }
    }
    Ok(())
}
