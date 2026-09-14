use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WriteEvent {
    pub ts: String,
    pub path: String,
    pub before: Option<String>,
    pub after: String,
    pub actor: String,
    pub reason: String,
}

pub struct WriteLog { path: PathBuf }

impl WriteLog {
    pub fn open(root: impl Into<PathBuf>) -> std::io::Result<Self> {
        let mut p: PathBuf = root.into();
        std::fs::create_dir_all(&p)?;
        p.push("writes.jsonl");
        Ok(Self { path: p })
    }
    pub fn append(&self, ev: &WriteEvent) -> std::io::Result<()> {
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&self.path)?;
        writeln!(f, "{}", serde_json::to_string(ev)?)?;
        Ok(())
    }
    pub fn read_all(&self) -> std::io::Result<Vec<WriteEvent>> {
        if !self.path.exists() { return Ok(Vec::new()); }
        Ok(std::fs::read_to_string(&self.path)?
            .lines()
            .filter(|l| !l.trim().is_empty())
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect())
    }
}
