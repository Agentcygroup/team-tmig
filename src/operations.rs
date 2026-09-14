use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
pub struct Finding {
    pub sites: usize,
    pub pattern: String,
}

#[derive(Serialize, Deserialize)]
pub struct Violation {
    pub total: usize,
    pub files: usize,
}

pub fn construction(pattern: &str, root: &Path) -> anyhow::Result<Finding> {
    let re = Regex::new(pattern)?;
    let mut sites = 0;
    for e in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !e.file_type().is_file() {
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(e.path()) {
            sites += re.find_iter(&text).count();
        }
    }
    Ok(Finding {
        sites,
        pattern: pattern.to_string(),
    })
}

pub fn coverage(invariant: &str, root: &Path) -> anyhow::Result<Violation> {
    let re = Regex::new(invariant)?;
    let mut total = 0;
    let mut files = 0;
    for e in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !e.file_type().is_file() {
            continue;
        }
        if let Ok(text) = std::fs::read_to_string(e.path()) {
            let n = re.find_iter(&text).count();
            if n > 0 {
                files += 1;
                total += n;
            }
        }
    }
    Ok(Violation { total, files })
}

pub fn fan(intent: &str, nodes: usize) -> anyhow::Result<[u8; 32]> {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    for i in 0..nodes {
        h.update(intent.as_bytes());
        h.update((i as u64).to_be_bytes());
    }
    Ok(h.finalize().into())
}
