use sha2::{Digest, Sha256};
use std::path::Path;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn file_hash(path: &Path) -> anyhow::Result<String> {
    let bytes = std::fs::read(path)?;
    Ok(hex(&Sha256::digest(&bytes)))
}

fn main() -> anyhow::Result<()> {
    let root = std::env::current_dir()?;
    let out = root.join("target/provenance");
    std::fs::create_dir_all(&out)?;

    let source_hash = file_hash(&root.join("Cargo.lock")).unwrap_or_else(|_| "missing".into());
    let recipe = serde_json::json!({
        "tool": "cargo",
        "command": "cargo build --release",
        "manifest": "Cargo.toml"
    });
    let recipe_bytes = serde_json::to_vec(&recipe)?;
    let build_recipe_hash = hex(&Sha256::digest(&recipe_bytes));

    let artifact = root.join("target/release/tmig");
    let artifact_hash = file_hash(&artifact).unwrap_or_else(|_| "not-built".into());
    let slsa = serde_json::json!({
        "schema": "tmig/provenance/slsa-lite/v1",
        "artifact": artifact.display().to_string(),
        "artifact_hash": artifact_hash,
        "source_hash": source_hash,
        "build_recipe_hash": build_recipe_hash,
        "timestamp": chrono::Utc::now(),
        "transform": recipe,
    });
    std::fs::write(out.join("slsa.json"), serde_json::to_vec_pretty(&slsa)?)?;

    let lock = std::fs::read_to_string(root.join("Cargo.lock")).unwrap_or_default();
    let packages = lock
        .lines()
        .filter_map(|line| line.strip_prefix("name = "))
        .map(|name| {
            serde_json::json!({
                "package": name.trim_matches('"'),
                "provenance": "Cargo.lock",
            })
        })
        .collect::<Vec<_>>();
    let sbom = serde_json::json!({
        "schema": "tmig/sbom/minimal/v1",
        "source": "Cargo.lock",
        "source_hash": file_hash(&root.join("Cargo.lock")).unwrap_or_else(|_| "missing".into()),
        "packages": packages,
    });
    std::fs::write(out.join("sbom.json"), serde_json::to_vec_pretty(&sbom)?)?;

    println!("wrote {}", out.join("slsa.json").display());
    println!("wrote {}", out.join("sbom.json").display());
    Ok(())
}
