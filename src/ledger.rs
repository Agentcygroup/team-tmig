use crate::roles::TeamRoles;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize)]
pub struct Intent {
    pub kind: String,
    pub body: serde_json::Value,
    pub ts: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub hash: String,
    pub ts: DateTime<Utc>,
    pub intent_hash: String,
    pub result_hash: String,
    pub signatures: Signatures,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Signatures {
    pub architect: Option<String>,
    pub operator: Option<String>,
    pub auditor: Option<String>,
    pub adversary: Option<String>,
}

pub fn hash_bytes(data: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().into()
}

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{:02x}", x)).collect()
}

impl Intent {
    pub fn hash(&self) -> [u8; 32] {
        hash_bytes(serde_json::to_string(self).unwrap_or_default().as_bytes())
    }
}

impl LedgerEntry {
    pub fn recompute_hash(&self) -> String {
        let mut h = Sha256::new();
        h.update(self.intent_hash.as_bytes());
        h.update(self.result_hash.as_bytes());
        h.update(self.ts.to_rfc3339().as_bytes());
        hex(&h.finalize())
    }
}

pub struct Ledger {
    path: PathBuf,
    entries: Vec<LedgerEntry>,
}

pub struct AuditReport {
    pub total: usize,
    pub bad: usize,
}

impl Ledger {
    pub fn open() -> anyhow::Result<Self> {
        let dir = std::env::var("HOME").map(PathBuf::from)?.join(".tmig");
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("ledger.jsonl");
        let mut entries = Vec::new();
        if path.exists() {
            let f = File::open(&path)?;
            for line in BufReader::new(f).lines() {
                let line = line?;
                if line.is_empty() {
                    continue;
                }
                // bounded
                if let Ok(e) = serde_json::from_str::<LedgerEntry>(&line) {
                    entries.push(e);
                }
            }
        }
        Ok(Self { path, entries })
    }
    pub fn append(&mut self, mut e: LedgerEntry) -> anyhow::Result<String> {
        e.hash = e.recompute_hash();
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        writeln!(f, "{}", serde_json::to_string(&e)?)?;
        let hash = e.hash.clone();
        self.entries.push(e);
        Ok(hash)
    }
    pub fn entries(&self) -> &[LedgerEntry] {
        &self.entries
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn verify_hashes(&self) -> anyhow::Result<AuditReport> {
        let mut bad = 0;
        for e in &self.entries {
            if e.recompute_hash() != e.hash {
                bad += 1;
            }
        }
        Ok(AuditReport {
            total: self.entries.len(),
            bad,
        })
    }
    pub fn rollback_to(&mut self, hash: &str) -> anyhow::Result<()> {
        let idx = self
            .entries
            .iter()
            .position(|e| e.hash == hash)
            .ok_or_else(|| anyhow::anyhow!("hash not in ledger"))?;
        self.entries.truncate(idx + 1);
        let mut f = File::create(&self.path)?;
        for e in &self.entries {
            writeln!(f, "{}", serde_json::to_string(e)?)?;
        }
        Ok(())
    }
    pub fn sign(&self, hash: &[u8; 32], roles: &TeamRoles) -> Signatures {
        Signatures {
            architect: roles.architect.signs(hash).map(|s| hex(&s)),
            operator: roles.operator.signs(hash).map(|s| hex(&s)),
            auditor: roles.auditor.signs(hash).map(|s| hex(&s)),
            adversary: roles.adversary.signs(hash).map(|s| hex(&s)),
        }
    }
}
