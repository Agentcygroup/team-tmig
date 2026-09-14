#!/usr/bin/env python3
"""Emit a Declaration for every capability listed in docs/CAPABILITIES.md
that does not already have one."""
import pathlib, json, hashlib, datetime

ROOT = pathlib.Path.home() / "team-tmig"
DECL = ROOT / "declarations"
DECL.mkdir(exist_ok=True)

def h(p):
    f = ROOT / p
    return hashlib.sha256(f.read_bytes()).hexdigest() if f.exists() else "missing"

# capability_id -> (evidence_path, subject)
capabilities = {
    "A1-author-proposal":    ("src/bin/hitl.rs", "propose an action with consequence class"),
    "A2-author-doctrine":    ("docs/DOCTRINE.md", "append a doctrine section"),
    "A3-author-catalog":     ("catalog/local-models.json", "declare the model catalog"),
    "A4-author-declaration": ("declarations", "emit a declaration with evidence hash"),
    "A5-author-primitive":   ("src/hitl/primitives.rs", "annotate an event with a primitive"),
    "B1-approve-sign":       ("src/hitl/approval.rs", "sign an approval with ed25519"),
    "B2-approve-two-person": ("src/hitl/approval.rs", "require two distinct roles for Legal"),
    "C1-execute-approved":   ("src/hitl/executor.rs", "execute an approved proposal"),
    "C2-refuse-financial":   ("src/hitl/executor.rs", "refuse auto-execute for Transfer"),
    "C3-refuse-legal":       ("src/hitl/executor.rs", "refuse auto-execute for Sign"),
    "D1-verify-record":      ("src/hitl/executor.rs", "record the outcome of an execution"),
    "D2-audit-standards":    ("target/standards/audit.txt", "audit the tree against claimed standards"),
    "D3-declaration-hashes": ("tests/cross_layer.rs", "verify declaration evidence hashes"),
    "E1-ledger-append":      ("src/hitl/ledger.rs", "append a content-addressed event"),
    "E2-ledger-read":        ("src/hitl/ledger.rs", "read the ledger"),
    "E3-ledger-revoke":      ("src/hitl/ledger.rs", "revoke a prior event by hash"),
    "E4-ledger-annotate":    ("src/hitl/ledger.rs", "attach retention/disclosure/provenance"),
    "F1-plane-derive":       ("src/bin/cp-hash.rs", "derive the control plane hash"),
    "F2-plane-admit":        ("src/bin/cp-attest.rs", "admit or reject an action proof"),
    "F3-reject-out-of-scope":("src/controlplane/mod.rs", "reject out-of-scope proofs"),
    "F4-reject-no-approval": ("src/controlplane/mod.rs", "reject proofs with no approvals"),
    "G1-provenance-emit":    ("target/provenance/slsa.json", "emit SLSA L1 provenance"),
    "G2-sbom-emit":          ("target/provenance/sbom.json", "emit a CycloneDX SBOM"),
    "G3-provenance-verify":  ("build.rs", "re-hash source and compare to slsa.json"),
    "H1-catalog-models":     ("catalog/local-models.json", "declare the model scope"),
    "H2-catalog-standards":  ("catalog/standards-scope.json", "declare the standards scope"),
    "H3-catalog-tags":       ("catalog/tag-vocabulary.json", "declare the tag vocabulary"),
    "H4-catalog-composites": ("catalog/combinatorial-expansion.json", "declare the composites"),
    "I1-audit-standards":    ("target/standards/audit.txt", "clause-by-clause standard audit"),
    "I2-check-boundary":     ("~/.pi/agent/APPEND_SYSTEM.md", "verify the host boundary"),
    "I3-verify-backfill":    ("~/.tmig/hitl.jsonl", "scan for unverified executions"),
    "I4-mastery-map":        ("target/provenance/mastery-map.json", "specialization over experience"),
    "J1-repro-build":        ("Cargo.toml", "rebuild the tree from source"),
    "J2-repro-test":         ("target/release/deps", "run all lib tests"),
    "J3-repro-clean-build":  ("Cargo.lock", "rebuild without caches"),
}

ts = datetime.datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ")
emitted = 0
for cap_id, (path, subject) in capabilities.items():
    name = f"{cap_id}.json"
    out = DECL / name
    # Don't clobber a declaration whose hash is already correct.
    hh = h(path.replace("~/", str(pathlib.Path.home()) + "/"))
    payload = {
        "claim_id": cap_id,
        "subject": subject,
        "evidence_path": path,
        "evidence_hash": hh,
        "ts": ts,
    }
    if out.exists():
        cur = json.loads(out.read_text())
        if cur.get("evidence_hash") == hh:
            continue
    out.write_text(json.dumps(payload, indent=2))
    emitted += 1
    print(f"{name}: {hh[:16]}")

print(f"emitted: {emitted}")
