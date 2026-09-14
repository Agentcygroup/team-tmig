# Threat Model

Assets source code audit log crypto keys build artifacts
 Adversaries A1 outside A2 insider read A3 insider write A4 supply chain A5 state level A6 scanner
 Attack surfaces build pipeline source repo audit log signed artifacts runtime
 Mitigations SHA-256 hashing pre-commit hook no unsafe outside src/ffi
rs pinned Cargo
lock
 Mitigations absent signed builds SBOM fuzzing dependency scanning key rotation
 Residual risk A4 and A5 unmitigated
