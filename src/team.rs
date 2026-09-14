use crate::ledger::{hash_bytes, hex, Intent, Ledger, LedgerEntry};
use crate::operations;
use crate::roles::TeamRoles;

pub struct TeamTMIG {
    pub roles: TeamRoles,
    pub ledger: Ledger,
}

impl TeamTMIG {
    pub fn init() -> anyhow::Result<Self> {
        let roles = TeamRoles::generate();
        let ledger = Ledger::open()?;
        let dir = std::env::var("HOME")
            .map(std::path::PathBuf::from)?
            .join(".tmig/roles");
        std::fs::create_dir_all(&dir)?;
        for (name, role) in [
            ("architect", &roles.architect),
            ("operator", &roles.operator),
            ("auditor", &roles.auditor),
            ("adversary", &roles.adversary),
        ] {
            std::fs::write(dir.join(format!("{}.pub", name)), hex(&role.verifying))?;
        }
        Ok(Self { roles, ledger })
    }
    pub fn set_intent(&mut self, intent: Intent) -> anyhow::Result<String> {
        let ih = intent.hash();
        let entry = LedgerEntry {
            hash: String::new(),
            ts: chrono::Utc::now(),
            intent_hash: hex(&ih),
            result_hash: hex(&[0u8; 32]),
            signatures: self.ledger.sign(&ih, &self.roles),
        };
        self.ledger.append(entry)
    }
    pub fn execute(
        &mut self,
        intent_hash: &str,
        body: &serde_json::Value,
    ) -> anyhow::Result<String> {
        let kind = body.get("kind").and_then(|v| v.as_str()).unwrap_or("fan");
        let result = match kind {
            "construction" => {
                let pat = body.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
                let root = body.get("root").and_then(|v| v.as_str()).unwrap_or(".");
                serde_json::to_vec(&operations::construction(pat, std::path::Path::new(root))?)?
            }
            "coverage" => {
                let inv = body.get("invariant").and_then(|v| v.as_str()).unwrap_or("");
                let root = body.get("root").and_then(|v| v.as_str()).unwrap_or(".");
                serde_json::to_vec(&operations::coverage(inv, std::path::Path::new(root))?)?
            }
            _ => {
                let cmd = body.get("command").and_then(|v| v.as_str()).unwrap_or("");
                let nodes = body.get("nodes").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
                operations::fan(cmd, nodes)?.to_vec()
            }
        };
        let rh = hash_bytes(&result);
        let entry = LedgerEntry {
            hash: String::new(),
            ts: chrono::Utc::now(),
            intent_hash: intent_hash.to_string(),
            result_hash: hex(&rh),
            signatures: self.ledger.sign(&rh, &self.roles),
        };
        self.ledger.append(entry)
    }
    pub fn audit(&self) -> anyhow::Result<crate::ledger::AuditReport> {
        self.ledger.verify_hashes()
    }
    pub fn red_team(&self) -> anyhow::Result<usize> {
        let modes = crate::dynamics::all_modes();
        let mut held = 0;
        for m in &modes {
            if crate::dynamics::bound_holds(m) {
                held += 1;
            }
        }
        Ok(held)
    }
    pub fn rollback(&mut self, hash: &str) -> anyhow::Result<()> {
        self.ledger.rollback_to(hash)
    }
}
