# Team TMIG Doctrine

1. Content-addressed everything.
2. Reversibility is non-negotiable.
3. Bounded locality.
4. Monotone accumulation.
5. Two-person rule for critical operations.

## Boundary Predicates
P (thesis): Every consequential action must be proposed by a machine, approved by a human with a cryptographic signature, executed only within its consequence class, verified after execution, and recorded in a content-addressed ledger. Nothing is thrown away. Every state is reversible. Every decision is auditable.

S (scope): Developer tooling that operates on files, commits, tickets, and commands, in real time, at machine speed, with human authorization for consequential actions.

Only features in `F_P ∩ F_S` belong on the roadmap. See `docs/BOUNDARY.md` and `src/boundary.rs`.

Model-layer P: all model inference runs locally on Apple Silicon through MLX or Ollama; no model-provider network calls, CUDA, TensorRT, NGC account, or AI Enterprise license.

Model-layer S: developer tooling only — text/code/commands/results over files, commits, tickets, commands, reviews, tests, docs, and runbooks. See `docs/MODEL_BOUNDARY.md` and `src/model_boundary.rs`.

Control plane: stateless conformance engine with exactly three responsibilities: admit, route, attest. It is derived from doctrine, the local model catalog, and the skill taxonomy. See `docs/CONTROLPLANE.md` and `src/controlplane/mod.rs`.

Sub-arity: compliance, license, and certification claims reduce to six content-addressable primitives: declaration, consent, retention, disclosure, provenance, and revocation. See `docs/SUBARITY.md` and `src/subarity.rs`.

Tag arity: every review, issue, ledger, or doctrine tag collapses to one of eleven canonical tag sequences. See `docs/TAGS.md` and `src/tags.rs`.

Signal pipeline: imaging itself is outside substrate scope, but its transform grammar is admitted as five primitives: classification, feature extraction, multi-scale signal analysis, pattern recognition, and projection. See `docs/SIGNAL_PIPELINE.md` and `src/signal_pipeline.rs`.

## Roles
Architect · Operator · Auditor · Adversary

## Methods
set_intent · execute · audit · red_team · rollback

## Operations
construction · coverage · fan

## Skills
All agents carry a universal developer-superpower baseline: dorking, code whispering, knowledge retrieval, version-control navigation, bug prediction, root-cause debugging, API orchestration, framework fluency, refactoring, test generation, documentation, monitoring, security audit, responsible AI, and continuous learning.

Specialized skills are role-scoped and content-addressed in `src/skills.rs`. Black-hat/white-hat magic is represented only as authorized adversarial testing, threat modeling, and defensive validation under the existing invariants.

## Claim Grammar
Consciousness, superintelligence, takeover, escape, and lying are observations, not features. Their grammar and bounded reflection events live in `src/reflection.rs`; operator-facing notes live in `docs/CLAIM_GRAMMAR.md`.

Invariant: `reflection_depth <= 3`.

## Dynamics (30)
propagate migrate mimic linger veil observe seal watch inject swarm
carry drop dwell wait tick morph rewrite obscure wrap fold blur
pulse orchestrate siphon anchor slide rise scan open act

## Local model scope

**P (invariant).** All model inference runs locally, on Apple Silicon, through MLX or Ollama. No network calls to model providers. No CUDA. No NGC. No TensorRT. No AI Enterprise license. Every model exposes an HTTP/OpenAI-compatible or CLI interface.

**S (scope).** Developer tooling at the substrate level: text in, text out; code in, code out; commands in, results out. Vision, ASR/TTS, medical imaging, weather, protein folding, and codecs are out of scope unless a developer workflow explicitly requires them, and none do at the substrate level.

**F_P ∩ F_S.** The finite set of buildable models is `catalog/local-models.json`. Of the 831 NGC catalog entries, 20 survive. The 811 rejections fail P (need CUDA/TensorRT/NGC/AI Enterprise) or S (not developer tooling).

## Human authorization

Every consequential action is **human-authorized**. A proposal that would
change state must carry at least one signed approval before it can execute.
The control plane refuses to admit proofs with an empty approvals list when
this clause is present in the doctrine.

## Standards conformance

TMIG distinguishes **conformance by implementation** (testable against a
published specification, no external audit) from **certification by audit**
(ISO/IEC management systems requiring an external audit program).

### Conformance by implementation

- **IETF RFC 8259** — JSON. Every HITL ledger event is one JSON object per line.
- **IETF RFC 8032** — Ed25519. Every approval signature.
- **IETF RFC 6920** — `ni:` URI hash references. Ledger events reference each other by content hash.
- **IETF RFC 9110** — HTTP semantics. `cp-attest` returns a JSON object whose `ok` field follows the HTTP success/failure convention; exit code 0 = admit, 3 = reject.

### Self-attestation

- **NIST AI RMF 1.0** — the control plane's `P` (thesis) and `S` (scope) predicates are the Govern and Map functions for this substrate. The ledger is Measure. The rejection path is Manage.
- **NIST SP 800-218 (SSDF)** — the build-and-release process (`cargo build --release`, `cargo test --release`, signed commits in the future).

### Explicitly not claimed

- **ISO/IEC 27001** — requires an ISMS, Statement of Applicability, internal audit, and annual surveillance. Multi-year program. Not in scope.
- **ISO/IEC 42001** — AI management system. Same audit structure. Not in scope.
- **ISO/IEC 23894** — AI risk management. Overlaps 27001; not separately claimed.
- **IEEE 7000-series** — conceptual ethics frameworks. No certification mechanism.
- **W3C Verifiable Credentials / DID** — would apply if TMIG issued credentials; it does not.
- **SLSA, OpenSSF, C2PA** — adjacent supply-chain and provenance specs; not claimed.

### Out of scope by model scope

Domains excluded by `## Local model scope` are also excluded from standards
claims: HL7 / FHIR, PCI DSS Council, SMPTE, IEC TC 65, automotive functional
safety (ISO 26262), medical device software (IEC 62304).

The standards scope is filtered by the same `P` and `S` as the model catalog:
a standards body is in scope iff conformance is content-addressable, auditable,
and bears on developer tooling at the substrate level.

## Compliance, audit, license, certification scope

**Sub-arity.** Every in-scope compliance regime, license, and certification
reduces to one of six primitives defined in `src/hitl/primitives.rs`:
Declaration, Consent, Retention, Disclosure, Provenance, Revocation.

**In scope (14):** GDPR, EU AI Act, CCPA/CPRA (compliance regimes);
Apache-2.0, MIT, BSD-3-Clause, MPL-2.0, AGPL-3.0, CC0-1.0 (licenses);
NIST AI RMF self-attest, NIST SSDF self-attest, SLSA L1-2, OpenChain,
SBOM CycloneDX (certifications).

**Out of scope (~311):** domains excluded by `## Local model scope`; any
regime requiring an external auditor (SOC 1/2/3, ISO 27001, ISO 42001,
ISO 27701, FedRAMP, StateRAMP, CMMC, HITRUST, PCI DSS); any license not
OSI-approved.

**Audit bodies (0):** none engaged.

## Tag vocabulary

Every CS term in scope is a sequence of tags from a fixed 36-tag vocabulary
derived from nine concerns x four modes:

    struct:{type,op,inv,prop}
    behav:{type,op,inv,prop}
    state:{type,op,inv,prop}
    comm:{type,op,inv,prop}
    correct:{type,op,inv,prop}
    perf:{type,op,inv,prop}
    sec:{type,op,inv,prop}
    evolve:{type,op,inv,prop}
    meaning:{type,op,inv,prop}

**In scope: 34 of 36.** Out of scope: `comm:type`, `comm:prop`
(network-layer specifics; TMIG uses HTTP and filesystem, expressed as
`comm:op` and `comm:inv`).

**Canonical tags (11):** feat, fix, refactor, perf, docs, test, build, ci,
chore, sec, revert. Each is a fixed sequence over the 36. There are ~120
observed surface variants. The normalizer maps surface -> canonical.

**Out-of-scope surfaces (14):** wip, experiment, spike, poc, proto, nit
(fail P: no content-addressed terminal state); release, merge, cherry-pick,
squash, rebase, BREAKING CHANGE, [skip ci], style (fail S: not a substrate
concern).

Every canonical tag is a Declaration. Every normalizer decision is a
ledger event. The vocabulary is closed: no CS term requires a 37th tag.

## Combinatorial expansion

The four fixed vocabularies (36 generative tags, 6 primitives, 54 metrics,
7 layers) form a closed language. The syntactic cross-product is
36 × 6 × 54 × 7 = 81,648 cells.

Three constraints reduce the syntactic product:

1. **Domain coherence** — tag concern must match metric domain.
2. **Primitive attachment** — the primitive must attach to that domain.
3. **Layer consistency** — the primitive must be meaningful at that layer.

A fourth constraint — semantic distinctness — reduces the result to **45
distinct composite concepts**, each a four-tuple (tag, primitive, metric,
layer) naming a queryable claim.

The 45 composites are enumerated in `catalog/combinatorial-expansion.json`
and in `src/combinatorial/composites.rs`. Every composite is a Declaration
at layer 2 (Ledger), with the primitive that satisfies it, the metric that
measures it, and the tag that names it.

The other ~898,083 cells are noise and are not enumerated.
