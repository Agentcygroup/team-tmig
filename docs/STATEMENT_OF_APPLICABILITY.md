# Statement of Applicability

| Control | Status | Evidence |
|---|---|---|
| Build reproducibility | applied | Cargo.lock, `pipeline/01-hermetic.sh` |
| Supply-chain audit | applied | `sec/02-cve.sh`, `sec/12-sbom.sh` |
| License compliance | applied | `deny.toml`, `cargo deny check` |
| Boundary declaration | applied | `mastery/05-boundary-declared.sh` |
| Self-reference closure | applied | `mastery/04-self-reference.sh` |
| Subarity | applied | `mastery/06-subarity.sh` |
| Provenance chain | applied | `.audit/log/`, `mastery/02-provenance-chain.sh` |
| Signed releases | applied | `.github/workflows/release.yml`, cosign |
