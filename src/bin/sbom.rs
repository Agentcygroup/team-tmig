//! Emits a minimal CycloneDX 1.5 SBOM from Cargo.lock.
//! Usage: sbom > target/provenance/sbom.json

use std::fs;

fn license_of(name: &str) -> Option<String> {
    match name {
        "serde" | "serde_json" | "clap" | "anyhow" | "chrono" | "sha2" | "digest"
        | "block-buffer" | "crypto-common" | "cpufeatures" | "generic-array" | "typenum"
        | "cfg-if" | "libc" => Some("MIT OR Apache-2.0".into()),
        "ed25519-dalek" | "ed25519" => Some("BSD-3-Clause".into()),
        _ => None,
    }
}

fn main() {
    debug_assert!(!std::env::args().any(|a| a.contains('\0')));
    let lock = match fs::read_to_string("Cargo.lock") {
        Ok(s) => s,
        Err(_) => {
            eprintln!("no Cargo.lock; run `cargo build` first");
            std::process::exit(2);
        }
    };

    let mut components = Vec::new();
    let mut name = String::new();
    let mut version = String::new();
    let mut in_package = false;

    for line in lock.lines() {
        if line == "[[package]]" {
            if in_package && !name.is_empty() {
                {
                    let lic = license_of(&name);
                    let component = match lic {
                        Some(l) => format!(
                            r#"    {{"type":"library","name":"{}","version":"{}","licenses":[{{"license":{{"id":"{}"}}}}]}}"#,
                            name, version, l
                        ),
                        None => format!(
                            r#"    {{"type":"library","name":"{}","version":"{}","licenses":[{{"license":{{"name":"NOASSERTION"}}}}]}}"#,
                            name, version
                        ),
                    };
                    components.push(component);
                }
            }
            name.clear();
            version.clear();
            in_package = true;
        } else if let Some(v) = line.strip_prefix("name = ") {
            name = v.trim_matches('"').to_string();
        } else if let Some(v) = line.strip_prefix("version = ") {
            version = v.trim_matches('"').to_string();
        }
    }
    if in_package && !name.is_empty() {
        {
            let lic = license_of(&name);
            let component = match lic {
                Some(l) => format!(
                    r#"    {{"type":"library","name":"{}","version":"{}","licenses":[{{"license":{{"id":"{}"}}}}]}}"#,
                    name, version, l
                ),
                None => format!(
                    r#"    {{"type":"library","name":"{}","version":"{}","licenses":[{{"license":{{"name":"NOASSERTION"}}}}]}}"#,
                    name, version
                ),
            };
            components.push(component);
        }
    }

    let ts = std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into());

    println!(
        r#"{{"bomFormat":"CycloneDX","specVersion":"1.5","version":1,"metadata":{{"timestamp":"{}"}},"components":["#,
        ts
    );
    println!("{}", components.join(",\n"));
    println!("]}}");
}
