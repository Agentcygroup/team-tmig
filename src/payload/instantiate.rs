use crate::payload::registry::{Kind, PayloadSpec};

pub fn instantiate(spec: &PayloadSpec, seed: &[u8]) -> Vec<u8> {
    let mut h = sha2::Sha256::new();
    use sha2::Digest;
    h.update(spec.name.as_bytes());
    h.update(seed);
    let d = h.finalize();
    match spec.kind {
        Kind::Json | Kind::Manifest | Kind::Index | Kind::Receipt | Kind::Sbom => {
            format!("{{\"name\":\"{}\",\"kind\":\"{:?}\",\"seed\":\"{}\"}}",
                spec.name, spec.kind, hex(&d[..8])).into_bytes()
        }
        Kind::Toml | Kind::Config => {
            format!("[{}]\nseed = \"{}\"\n", spec.name, hex(&d[..8])).into_bytes()
        }
        Kind::Yaml | Kind::CiWorkflow | Kind::CdPipeline | Kind::Compose | Kind::Helm | Kind::Kustomize | Kind::Ansible => {
            format!("name: {}\nseed: {}\n", spec.name, hex(&d[..8])).into_bytes()
        }
        Kind::Log | Kind::Audit | Kind::Ledger => {
            format!("{{\"ts\":\"2026-09-14T00:00:00Z\",\"name\":\"{}\",\"seed\":\"{}\"}}\n",
                spec.name, hex(&d[..8])).into_bytes()
        }
        Kind::Signature | Kind::Key | Kind::Cert | Kind::Witness | Kind::Notary | Kind::Timestamp => {
            d.to_vec()
        }
        Kind::MerkleProof | Kind::InclusionProof | Kind::ConsistencyProof | Kind::TransparencyLog | Kind::Merkle => {
            format!("proof {} {}\n", spec.name, hex(&d[..16])).into_bytes()
        }
        Kind::Terraform => {
            format!("resource \"tmig_{}\" \"x\" {{ seed = \"{}\" }}\n",
                spec.name.replace('.', "_"), hex(&d[..8])).into_bytes()
        }
        Kind::Dockerfile => {
            format!("FROM scratch\nLABEL tmig.name={}\nLABEL tmig.seed={}\n",
                spec.name, hex(&d[..8])).into_bytes()
        }
        Kind::Archive | Kind::Bundle | Kind::Distribute | Kind::Mirror | Kind::Replica | Kind::Cdn | Kind::Torrent | Kind::Ipfs => {
            d.to_vec()
        }
        Kind::Diff | Kind::Patch | Kind::Migration | Kind::Rollback => {
            format!("--- a/{}\n+++ b/{}\n@@ -1 +1 @@\n-old\n+new {}\n",
                spec.name, spec.name, hex(&d[..8])).into_bytes()
        }
        Kind::Docs | Kind::Readme | Kind::Changelog | Kind::License | Kind::Docsite | Kind::Site => {
            format!("# {}\n\nseed {}\n", spec.name, hex(&d[..8])).into_bytes()
        }
        _ => d.to_vec(),
    }
}

fn hex(b: &[u8]) -> String { b.iter().map(|x| format!("{:02x}", x)).collect() }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::payload::registry::Registry;
    #[test] fn instantiates_all() {
        let r = Registry::omega();
        for s in &r.specs {
            let b = instantiate(s, b"seed");
            assert!(!b.is_empty(), "empty for {}", s.name);
        }
    }
}
