# Conformance

## What "green" means here
Green = `cargo fmt --check && cargo clippy -D warnings && cargo test` all pass
**and** the receipt at `target/provenance/green-receipt.txt` matches the tagged commit.

## Proven vs asserted
- **Proven** (verifiable from the tree, alone): build, tests, lints, deny, boundaries, subarity, self-reference, receipt.
- **Asserted** (verified by external systems, not by `cargo xt ci`): SBOM attachment, SLSA provenance, cosign signatures, HTTP header policy.
Each is checked by a named script under `checks/mastery/`.
