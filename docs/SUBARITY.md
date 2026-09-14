# Sub-arity

Sub-arity reduces compliance, license, and certification names to the smallest content-addressable unit each one needs.

There are exactly six primitives:

1. Declaration — `{claim, subject, timestamp, hash}`
2. Consent — `{subject_id, action_hash, signature, timestamp, purpose}`
3. Retention — `{event_hash, ttl, deletion_policy}`
4. Disclosure — `{event_hash, recipient, purpose}`
5. Provenance — `{artifact_hash, source_hash, transform}`
6. Revocation — `{event_hash, negated_by, reason}`

The 14 in-scope regimes/licenses/certifications collapse to these six primitives. Two were already present as prose/hashes; this tree now adds machine-readable support for the missing pieces:

- `Retention`, `Disclosure`, `Purpose`, `Declaration`, `Provenance` in `src/hitl/primitive.rs`
- optional retention/disclosure fields on every `HitlEvent`
- `HitlEvent::Revoked`
- `Approval.purpose`
- declaration files in `declarations/*.json`
- build provenance/SBOM generator: `cargo run --release --bin provenance`
- derivation table in `src/subarity.rs`

The 14 items are now queries over ledger fields and declaration/provenance files, not 14 subsystems.
