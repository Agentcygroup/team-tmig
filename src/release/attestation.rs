use crate::release::{Provenance, Sbom, Signature};

#[derive(Clone, Debug)]
pub struct Attestation {
    pub provenance: Provenance,
    pub sbom: Sbom,
    pub signature: Signature,
}

impl Attestation {
    pub fn build(p: Provenance, s: Sbom, sig: Signature) -> Self {
        Self {
            provenance: p,
            sbom: s,
            signature: sig,
        }
    }
    pub fn to_json(&self) -> String {
        format!(
            "{{\"level\":{},\"sbom\":{}}}",
            self.provenance.level,
            self.sbom.to_json()
        )
    }
}
