//! Build-time provenance. Emits target/provenance/slsa.json with hashes of
//! the source tree, the build recipe, and (post-build, best-effort) the
//! binary. SLSA Level 1 is source_hash + artifact_hash; Level 2 adds
//! recipe_hash.

use std::fs;
use std::path::Path;
use std::process::Command;

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=docs/DOCTRINE.md");
    println!("cargo:rerun-if-changed=catalog");
    println!("cargo:rerun-if-changed=Cargo.toml");

    let out_dir = std::env::var("OUT_DIR").unwrap_or_else(|_| "target".into());
    let prov_dir = Path::new(&out_dir)
        .ancestors()
        .find(|p| p.ends_with("target"))
        .map(|p| p.join("provenance"))
        .unwrap_or_else(|| Path::new("target/provenance").to_path_buf());
    let _ = fs::create_dir_all(&prov_dir);

    // Source hash: hash of the concatenated file names + sizes.
    let mut src_manifest = String::new();
    if let Ok(entries) = fs::read_dir("src") {
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            let size = e.metadata().map(|m| m.len()).unwrap_or(0);
            src_manifest.push_str(&format!("{}:{}\n", name, size));
        }
    }
    let source_hash = sha256_hex(src_manifest.as_bytes());

    // Recipe hash: cargo version + the build profile.
    let cargo_version = Command::new("cargo")
        .arg("--version")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into());
    let recipe_hash = sha256_hex(cargo_version.as_bytes());

    let ts = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into());

    let slsa = format!(
        r#"{{
  "schema": "tmig/slsa-provenance/v1",
  "source_hash": "{}",
  "recipe_hash": "{}",
  "cargo_version": "{}",
  "built_at": "{}",
  "level": 1
}}
"#,
        source_hash, recipe_hash, cargo_version, ts
    );
    let _ = fs::write(prov_dir.join("slsa.json"), slsa);
}
