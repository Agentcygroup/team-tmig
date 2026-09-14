# Statement of Applicability

| Control | Status | Evidence |
|---|---|---|
| Build reproducibility | applied | Cargo.lock, checks/pipeline/01-hermetic.sh |
| Supply-chain audit | applied | checks/sec/02-cve.sh, checks/sec/12-sbom.sh |
| License compliance | applied | deny.toml |
| Boundary declaration | applied | checks/mastery/05-boundary-declared.sh |
| Self-reference closure | applied | checks/mastery/04-self-reference.sh |
| Subarity | applied | checks/mastery/06-subarity.sh |
| Provenance chain | applied | .audit/log/, checks/mastery/02-provenance-chain.sh |
| Signed releases | applied | .github/workflows/release.yml |
