# Declared Clause Set

Clauses shall be interpreted according to RFC 2119.
A clause that cannot be tested should be marked out of scope.

| ID | Source | Clause | Test |
|----|--------|--------|------|
| C1 | Rust Reference | build succeeds | cargo build --release |
| C2 | internal | at least 3 binaries | ls target/release |
| C3 | internal | at least 30 modules | find src -name mod.rs |
| C4 | NIST FIPS 180-4 | SHA-256 dependency | grep sha2 |
| C5 | ISO 27001 A.8.15 | audit dir exists | test -d .audit |
| C6 | internal | lib tests pass | cargo test --lib |
| C7 | internal | providers declared | grep Provider |
| C8 | internal | claims module present | test -f verify.rs |
| C9 | ISO 15288 §6.4.7 | interface docs | cargo doc |
