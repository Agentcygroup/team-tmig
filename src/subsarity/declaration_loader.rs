use crate::hitl::Declaration;
use std::path::Path;

pub fn load_declarations(dir: &Path) -> anyhow::Result<Vec<Declaration>> {
    let mut out = Vec::new();
    if !dir.exists() { return Ok(out); }
    for entry in std::fs::read_dir(dir)? {
        let e = entry?;
        let p = e.path();
        if p.extension().and_then(|s| s.to_str()) != Some("json") { continue; }
        let text = std::fs::read_to_string(&p)?;
        let v: serde_json::Value = serde_json::from_str(&text)?; // bounded
        out.push(Declaration {
            claim_id: v.get("claim_id").and_then(|x| x.as_str()).unwrap_or("").into(),
            subject: v.get("subject").and_then(|x| x.as_str()).unwrap_or("").into(),
            evidence_path: v.get("evidence_path").and_then(|x| x.as_str()).unwrap_or("").into(),
            evidence_hash: v.get("evidence_hash").and_then(|x| x.as_str()).unwrap_or("").into(),
            ts: v.get("ts").and_then(|x| x.as_str()).unwrap_or("").into(),
        });
    }
    Ok(out)
}
