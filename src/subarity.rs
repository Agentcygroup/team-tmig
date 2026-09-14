use crate::ledger::{hash_bytes, hex};
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Primitive {
    Declaration,
    Consent,
    Retention,
    Disclosure,
    Provenance,
    Revocation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SubArityRow {
    pub item: &'static str,
    pub unit: &'static str,
    pub primitives: &'static [Primitive],
    pub in_tree: &'static str,
}

impl SubArityRow {
    pub fn hash(&self) -> String {
        hex(&hash_bytes(
            &serde_json::to_vec(self).expect("sub-arity row serializes"),
        ))
    }
}

pub fn table() -> Vec<SubArityRow> {
    use Primitive::*;
    vec![
        SubArityRow { item: "GDPR", unit: "Consent(subject,purpose,action_hash)+Retention(event,ttl)+Revocation(event,negated_by)", primitives: &[Consent, Retention, Revocation], in_tree: "approval purpose + retention + revocation" },
        SubArityRow { item: "EU AI Act", unit: "Declaration(conformity)+Provenance(artifact,source,transform)+Disclosure(event,recipient,purpose)", primitives: &[Declaration, Provenance, Disclosure], in_tree: "declarations + provenance + disclosure" },
        SubArityRow { item: "CCPA/CPRA", unit: "Consent+Retention+Disclosure(sale)+Revocation", primitives: &[Consent, Retention, Disclosure, Revocation], in_tree: "approval purpose + retention + disclosure + revocation" },
        SubArityRow { item: "Apache-2.0", unit: "Provenance(artifact,license_text,notice_file)", primitives: &[Provenance], in_tree: "build provenance/SBOM" },
        SubArityRow { item: "MIT", unit: "Provenance(artifact,license_text)", primitives: &[Provenance], in_tree: "build provenance/SBOM" },
        SubArityRow { item: "BSD-3-Clause", unit: "Provenance(artifact,license_text)", primitives: &[Provenance], in_tree: "build provenance/SBOM" },
        SubArityRow { item: "MPL-2.0", unit: "Provenance(file,license_text,per_file_notice)", primitives: &[Provenance], in_tree: "build provenance/SBOM" },
        SubArityRow { item: "AGPL-3.0", unit: "Provenance(artifact,license_text)+Disclosure(network_user,source_offer)", primitives: &[Provenance, Disclosure], in_tree: "build provenance/SBOM + disclosure" },
        SubArityRow { item: "CC0-1.0", unit: "Declaration(public_domain,artifact,timestamp)", primitives: &[Declaration], in_tree: "declarations/*.json" },
        SubArityRow { item: "NIST AI RMF self-attest", unit: "Declaration(function,mapping,timestamp)", primitives: &[Declaration], in_tree: "declarations/*.json" },
        SubArityRow { item: "NIST SSDF self-attest", unit: "Declaration(practice_id,evidence_hash,timestamp)", primitives: &[Declaration], in_tree: "declarations/*.json" },
        SubArityRow { item: "SLSA L1-L2", unit: "Provenance(build_artifact,source_hash,build_recipe_hash)", primitives: &[Provenance], in_tree: "target/provenance/slsa.json" },
        SubArityRow { item: "OpenChain", unit: "Declaration(license_id,sbom_hash,timestamp)+Provenance(dependency)", primitives: &[Declaration, Provenance], in_tree: "declarations + SBOM" },
        SubArityRow { item: "SBOM CycloneDX/SPDX", unit: "Provenance(package,version,license,hash)", primitives: &[Provenance], in_tree: "target/provenance/sbom.json" },
    ]
}

pub fn primitive_union() -> Vec<Primitive> {
    use std::collections::BTreeSet;
    table()
        .into_iter()
        .flat_map(|row| row.primitives.iter().copied())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub fn load_declarations(dir: &std::path::Path) -> anyhow::Result<Vec<crate::hitl::Declaration>> {
    let mut out = Vec::new();
    if !dir.exists() {
        return Ok(out);
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            out.push(serde_json::from_slice(&std::fs::read(path)?)?);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fourteen_items_collapse_to_six_primitives() {
        assert_eq!(table().len(), 14);
        assert_eq!(primitive_union().len(), 6);
    }

    #[test]
    fn every_subarity_unit_is_content_addressable() {
        for row in table() {
            assert_eq!(row.hash().len(), 64);
            assert!(!row.unit.is_empty());
        }
    }

    #[test]
    fn no_seventh_primitive_exists() {
        assert_eq!(
            primitive_union(),
            vec![
                Primitive::Declaration,
                Primitive::Consent,
                Primitive::Retention,
                Primitive::Disclosure,
                Primitive::Provenance,
                Primitive::Revocation,
            ]
        );
    }
}
