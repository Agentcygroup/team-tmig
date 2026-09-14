# Conformance

Each row is `(standard, clause, level, evidence)`.

| Standard | Clause | Level | Evidence |
|---|---|---|---|
| ISO/IEC 25010 | Functional suitability | partial | 56+ tests, no spec |
| ISO/IEC 25010 | Maintainability | high | module separation, content-addressed artifacts |
| ISO/IEC 15408 (Common Criteria) | EAL1: functionally tested | met | `cargo test --release --lib` |
| ISO/IEC 15408 (Common Criteria) | EAL2: structurally tested | not met | no independent structural test report |
| IEC 61508 (SIL) | SIL 0 (not applicable) | — | no safety function |
| NIST SP 800-218 (SSDF) | PO: Prepare the Organization | partial | `DOCTRINE.md` exists |
| NIST SP 800-218 (SSDF) | PS: Protect the Software | partial | content-addressed, unsigned releases |
| NIST SP 800-218 (SSDF) | PW: Produce Well-Secured Software | partial | Rust, tests |
| NIST SP 800-218 (SSDF) | RV: Respond to Vulnerabilities | met | `SECURITY.md` |
| NIST AI RMF 1.0 | Govern | partial | control plane derived from doctrine |
| NIST AI RMF 1.0 | Map | met | `catalog/local-models.json` + `DOCTRINE.md` |
| NIST AI RMF 1.0 | Measure | partial | 56+ tests |
| NIST AI RMF 1.0 | Manage | partial | `AdmitError`, `ScopeFailure` |
| IETF RFC 8259 | JSONL ledger | conformant | every line is a JSON object |
| IETF RFC 8032 | Ed25519 | conformant | `ed25519-dalek` in `approval.rs` |
| IETF RFC 6920 | hash references | conformant | `hash_bytes` in `ledger.rs` |
| IETF RFC 9110 | HTTP semantics | partial | `cp-attest` ok/fail; exit 0/3 |
| W3C VC Data Model 2.0 | — | not implemented | out of scope |
| W3C DID Core | — | not implemented | out of scope |
| SLSA | Level 1: documented provenance | met | `build.rs` emits `slsa.json` |
| SLSA | Level 2: signed provenance | met | `slsa.json.sig` via ssh-keygen |
| SLSA | Level 3: hardened build platform | not met | no remote build service |
| SLSA | Level 4: two-party review | not met | one developer |
| OpenSSF Scorecard | License | met | `LICENSE` |
| OpenSSF Scorecard | Security-Policy | met | `SECURITY.md` |
| OpenSSF Scorecard | CI-Tests | met | `.github/workflows/ci.yml` |
| OpenSSF Scorecard | SAST | not met | no static analysis wired |
| OpenSSF Scorecard | Pinned-Dependencies | not met | `Cargo.lock` not pinned by hash |
| OpenSSF Scorecard | Signed-Releases | not met | releases unsigned |
| OpenSSF Scorecard | Fuzzing | not met | no fuzz target |
| ISO/IEC 27001 | ISMS | not claimed | requires external audit |
| ISO/IEC 42001 | AI management system | not claimed | requires external audit |
| GDPR | Art. 17 (right to erasure) | partial | `Revoked` event; no formal DSR process |
| EU AI Act | Art. 12 (record-keeping) | partial | content-addressed ledger |
| OpenChain | per-dependency license manifest | met | `sbom.json` with licenses |

## TLA+ proofs

| Spec | Theorem | Status |
|---|---|---|
| `spec/murmuration.tla` | `Safety` (no execution without two distinct fellows) | declared; TLC run pending |
| `spec/murmuration.tla` | `Liveness` (two-approval units are executable) | declared |

## Audit

Run `bash bin/capability-check.sh` for the 35-capability pass/fail table.
Run `bash bin/mastery-map.sh` for the specialization/trajectory map.
Run `tlc spec/murmuration.tla -config spec/murmuration.cfg` for the safety proof.

## Levels

- **EAL1** (Common Criteria).
- **SLSA Level 2** (signed provenance).
- **NIST SSDF Level 1** with full RV.
- **ISO/IEC 15408**: not certified.
- **ISO/IEC 27001, 42001**: not claimed.
