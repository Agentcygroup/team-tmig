# team-tmig agent notes

- Build with `cargo build --release`.
- Demo with `bin/tmig-demo.sh`.
- Swarm with `target/release/athletic-swarm --mode adaptive --agents 10000 --duration 10`.
- Run all checkers with `bin/swarm-full.sh src/swarm/agent.rs`.
- Agent skill taxonomy lives in `src/skills.rs`; every agent must include the universal skill baseline plus role-specific bounded skills.
- Claim grammar/reflection lives in `src/reflection.rs`; treat consciousness, superintelligence, takeover, escape, and lying as measured observations, never features.
- Boundary predicates live in `src/boundary.rs` and `docs/BOUNDARY.md`; only build features in `F_P ∩ F_S`.
- Model catalog predicates live in `src/model_boundary.rs` and `docs/MODEL_BOUNDARY.md`; local Apple Silicon developer models only.
- Control plane lives in `src/controlplane/mod.rs`; it only admits, routes, and attests. Do not add scheduler/planner/model-router behavior there.
- Sub-arity lives in `src/subarity.rs` and `src/hitl/primitives.rs`; express compliance/license/cert work as declaration, consent, retention, disclosure, provenance, or revocation.
- Tag arity lives in `src/tags/`; collapse review, issue, ledger, and doctrine tags to one of eleven canonical tag sequences.
- Signal-pipeline grammar lives in `src/signal_pipeline.rs`; do not build vision features unless they pass P/S, but reuse the five transform primitives for content-addressed artifact transforms.
