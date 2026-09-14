pub enum Feature { ContentAddressing, MerkleProof, Signature, Attestation, Provenance, Sbom, Metric, Trace, Log, Admission, Routing }
pub const ALL_FEATURES: &[Feature] = &[Feature::ContentAddressing];
