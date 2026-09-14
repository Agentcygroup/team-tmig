use std::path::PathBuf;

#[derive(Default)]
pub struct HitlLedger {
    pub path: PathBuf,
    pub entries: Vec<String>,
}

impl HitlLedger {
    pub fn open() -> anyhow::Result<Self> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        let mut p = PathBuf::from(home);
        p.push(".tmig");
        std::fs::create_dir_all(&p).ok();
        p.push("hitl.jsonl");
        Ok(Self {
            path: p,
            entries: Vec::new(),
        })
    }
    pub fn append(&mut self, e: String) -> anyhow::Result<()> {
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        writeln!(f, "{}", e)?;
        self.entries.push(e);
        Ok(())
    }
}
