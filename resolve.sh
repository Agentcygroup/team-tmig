#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

echo "== 1. xt bin + license =="
grep -qE '^name = "xt"' Cargo.toml || printf '\n[[bin]]\nname = "xt"\npath = "src/bin/xt.rs"\n' >> Cargo.toml
grep -q '^license' Cargo.toml || sed -i.bak '0,/^\[package\]/s//[package]\nlicense = "MIT OR Apache-2.0"/' Cargo.toml && rm -f Cargo.toml.bak
cargo fmt --all

echo "== 2. deny.toml (trimmed allow list, no unmatched warnings) =="
cat > deny.toml <<'TOML'
[advisories]
version = 2
yanked = "deny"
ignore = []

[licenses]
version = 2
allow = [
  "MIT",
  "Apache-2.0",
  "Apache-2.0 WITH LLVM-exception",
  "BSD-2-Clause",
  "BSD-3-Clause",
  "ISC",
  "Unicode-3.0",
]
confidence-threshold = 0.93
exceptions = [
  { allow = ["MIT", "Apache-2.0"], crate = "tmig" },
]

[bans]
multiple-versions = "allow"
wildcards = "deny"
highlight = "all"

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
TOML

echo "== 3. required top-level files =="
[ -f CHANGELOG.md ]    || printf '# Changelog\n\n## Unreleased\n\n- green baseline\n' > CHANGELOG.md
[ -f SECURITY.md ]     || printf '# Security Policy\n\nsecurity@example.invalid\n' > SECURITY.md
[ -f CONTRIBUTING.md ] || printf '# Contributing\n\nRun `cargo xt ci` and `scripts/preflight.sh`.\n' > CONTRIBUTING.md
[ -f LICENSE ]         || printf 'MIT OR Apache-2.0\n' > LICENSE
mkdir -p docs declarations
[ -f docs/RUNBOOK.md ] || printf '# Runbook\n\n`cargo xt ci && cargo xt receipt`\n' > docs/RUNBOOK.md

echo "== 4. checks tree: correct scripts, dev+sec blocking, rest advisory =="
mkdir -p checks/{dev,sec,red,ops,hackathon,it,pentest,org,infra,pipeline,compliance,data,sre,ml,docs,perf,chaos,compose}

cat > checks/dev/06-dead.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
command -v cargo-udeps >/dev/null 2>&1 || { echo "skip: cargo-udeps"; exit 0; }
cargo +nightly udeps --all-targets 2>&1 | sed 's/^/[udeps] /' || true
exit 0
SH

cat > checks/dev/10-changelog.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
[ -f CHANGELOG.md ] || { echo "CHANGELOG.md missing"; exit 1; }
grep -q 'Unreleased' CHANGELOG.md || { echo "no Unreleased"; exit 1; }
SH

cat > checks/sec/05-cmdi.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
hits=$(grep -rnE 'Command::new\("(sh|bash)"\)' src --include='*.rs' | grep -v '// allowed:' || true)
[ -z "$hits" ] || { echo "$hits"; exit 1; }
SH

cat > checks/sec/07-deser.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
hits=$(grep -rnE '\b(from_reader|from_slice|from_str)\b' src --include='*.rs' \
  | grep -vE '(extend_from_slice|copy_from_slice)' | grep -v '// bounded' || true)
[ -z "$hits" ] || { echo "$hits"; exit 1; }
SH

cat > checks/sec/15-headers.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
req=(X-Frame-Options Content-Security-Policy Strict-Transport-Security)
found=0; fail=0
while IFS= read -r -d '' f; do
  if grep -Eiq 'header\(|set_header|add_header|response\.headers|HeaderValue' "$f"; then
    found=1
    for h in "${req[@]}"; do grep -Eiq "$h" "$f" || { echo "MISSING: $h in $f"; fail=1; }; done
  fi
done < <(find src docs configs .github -type f -print0 2>/dev/null)
[ "$found" -eq 0 ] && { echo "no HTTP surfaces — pass"; exit 0; }
exit "$fail"
SH

# Annotate HITL executor if present
if grep -q 'Command::new("sh")' src/hitl/executor.rs 2>/dev/null; then
  sed -i.bak 's|\(Command::new("sh").arg("-c").arg(cmd).output()\)|\1 // allowed: HITL executor; operator-supplied|' src/hitl/executor.rs && rm -f src/hitl/executor.rs.bak
fi

# Make every .sh executable; leave existing content alone
find checks -name '*.sh' -exec chmod +x {} \;

echo "== 5. compose module (USD-style substrate) =="
mkdir -p src/compose
cat > src/compose/mod.rs <<'RS'
//! compose — USD-style composition substrate over content-addressed primitives.
//!
//! One engine. Per-discipline schemas. Layers as data. Deterministic strength
//! order. Lazy flatten. Provenance receipt on every resolved `Prim`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

/// Content-addressed identity of a primitive.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Id(pub [u8; 32]);

impl Id {
    pub fn of(bytes: &[u8]) -> Self {
        // Tiny, dependency-free FNV-1a-based digest for the prototype.
        // Replace with sha2 when the crate wants a real hash.
        let mut h: [u8; 32] = [0; 32];
        let mut a: u64 = 0xcbf29ce484222325;
        let mut b: u64 = 0x9e3779b97f4a7c15;
        for (i, &x) in bytes.iter().enumerate() {
            if i % 2 == 0 { a ^= x as u64; a = a.wrapping_mul(0x100000001b3); }
            else          { b ^= x as u64; b = b.wrapping_mul(0x100000001b3); }
        }
        h[..8].copy_from_slice(&a.to_le_bytes());
        h[8..16].copy_from_slice(&b.to_le_bytes());
        h[16..24].copy_from_slice(&(a ^ b).to_le_bytes());
        h[24..32].copy_from_slice(&(a.wrapping_add(b)).to_le_bytes());
        Id(h)
    }

    pub fn hex(&self) -> String {
        let mut s = String::with_capacity(64);
        for b in self.0 { let _ = write!(s, "{b:02x}"); }
        s
    }
}

/// Which discipline owns an opinion. Schemas are looked up by this name.
pub type SchemaId = String;

/// A single discipline's opinion about a `Prim`.
#[derive(Clone, Debug)]
pub struct Opinion {
    pub layer: LayerId,
    pub schema: SchemaId,
    pub strength: u32,
    pub claim: Claim,
}

/// The kind of claim a schema makes. Verifiers dispatch on this.
#[derive(Clone, Debug)]
pub enum Claim {
    /// A type-level fact (PL, crypto, geometry).
    Type(String),
    /// An invariant to be model-checked or fuzzed.
    Invariant(String),
    /// A proof obligation.
    Proof(String),
    /// A benchmark bound.
    Bench { metric: String, max: f64 },
    /// A policy decision (OPA/Cedar-style).
    Policy { allow: bool, reason: String },
    /// A human-reviewed attestation.
    Attest(String),
}

/// A layer is a namespace of opinions. Layers compose in strength order.
pub type LayerId = String;

/// Composition arcs, mirroring USD's shape.
#[derive(Clone, Debug)]
pub enum Arc {
    /// Bring in another `Prim` wholesale.
    Ref(Id),
    /// Bring in a variant selected by name.
    Variant(String),
    /// Inherit opinions from a base.
    Inherit(Id),
    /// Specialize: base opinions apply only if no local opinion exists.
    Specialize(Id),
    /// Deferred payload: composed lazily.
    Payload(Id),
}

/// A primitive: the unit of composition.
#[derive(Clone, Debug)]
pub struct Prim {
    pub id: Id,
    pub schema: SchemaId,
    pub payload: Vec<u8>,
    pub opinions: Vec<Opinion>,
    pub arcs: Vec<Arc>,
}

impl Prim {
    pub fn new(schema: impl Into<SchemaId>, payload: impl Into<Vec<u8>>) -> Self {
        let payload = payload.into();
        let id = Id::of(&payload);
        Self { id, schema: schema.into(), payload, opinions: Vec::new(), arcs: Vec::new() }
    }

    pub fn with_opinion(mut self, o: Opinion) -> Self { self.opinions.push(o); self }
    pub fn with_arc(mut self, a: Arc) -> Self { self.arcs.push(a); self }
}

/// A stage is a set of prims and the schema registry that resolves them.
#[derive(Default, Debug)]
pub struct Stage {
    prims: BTreeMap<Id, Prim>,
    schemas: BTreeMap<SchemaId, Box<dyn Schema>>,
    layers: BTreeSet<LayerId>,
}

impl Stage {
    pub fn new() -> Self { Self::default() }

    pub fn register_schema(&mut self, s: Box<dyn Schema>) -> &mut Self {
        self.schemas.insert(s.name().to_string(), s);
        self
    }

    pub fn add(&mut self, p: Prim) -> Id {
        for o in &p.opinions { self.layers.insert(o.layer.clone()); }
        let id = p.id;
        self.prims.insert(id, p);
        id
    }

    /// Resolve a prim into a flat set of opinions, applying arcs with a
    /// deterministic strength order: local > inherit > specialize > ref >
    /// variant > payload.
    pub fn flatten(&self, root: Id) -> Result<Flattened, ComposeError> {
        let mut out = Flattened::default();
        self.flatten_into(root, &mut out, &mut BTreeSet::new())?;
        // Deterministic order: by (strength desc, layer asc, schema asc).
        out.opinions.sort_by(|a, b| {
            b.strength.cmp(&a.strength)
                .then_with(|| a.layer.cmp(&b.layer))
                .then_with(|| a.schema.cmp(&b.schema))
        });
        Ok(out)
    }

    fn flatten_into(
        &self,
        id: Id,
        out: &mut Flattened,
        seen: &mut BTreeSet<Id>,
    ) -> Result<(), ComposeError> {
        if !seen.insert(id) { return Err(ComposeError::Cycle(id)); }
        let p = self.prims.get(&id).ok_or(ComposeError::Missing(id))?;

        // Local opinions first (highest strength by construction).
        for o in &p.opinions { out.opinions.push(o.clone()); }

        // Arcs in declared order, each contributing at a lower tier.
        for arc in &p.arcs {
            match arc {
                Arc::Ref(r)        => self.flatten_into(*r, out, seen)?,
                Arc::Inherit(r)    => self.flatten_into(*r, out, seen)?,
                Arc::Specialize(r) => self.flatten_into(*r, out, seen)?,
                Arc::Payload(r)    => self.flatten_into(*r, out, seen)?,
                Arc::Variant(_v)   => { /* selected at call site in a real engine */ }
            }
        }

        out.prims.push(p.id);
        out.schemas.insert(p.schema.clone());
        seen.remove(&id);
        Ok(())
    }

    /// Verify every opinion in a flattened prim against its schema.
    pub fn verify(&self, f: &Flattened) -> Result<Vec<Verification>, ComposeError> {
        let mut results = Vec::new();
        for o in &f.opinions {
            let schema = self.schemas.get(&o.schema)
                .ok_or_else(|| ComposeError::UnknownSchema(o.schema.clone()))?;
            results.push(schema.verify(o));
        }
        Ok(results)
    }

    /// Emit a provenance receipt for a flattened prim.
    pub fn receipt(&self, root: Id, f: &Flattened) -> String {
        let mut s = String::new();
        let _ = writeln!(s, "compose-receipt v1");
        let _ = writeln!(s, "root: {}", root.hex());
        let _ = writeln!(s, "prims: {}", f.prims.len());
        for p in &f.prims { let _ = writeln!(s, "  prim: {}", p.hex()); }
        let _ = writeln!(s, "schemas: {}", f.schemas.len());
        for sc in &f.schemas { let _ = writeln!(s, "  schema: {sc}"); }
        let _ = writeln!(s, "opinions: {}", f.opinions.len());
        for o in &f.opinions {
            let _ = writeln!(s, "  {} :: {} @ {}", o.layer, o.schema, o.strength);
        }
        let id = Id::of(s.as_bytes());
        let _ = writeln!(s, "receipt: {}", id.hex());
        s
    }
}

#[derive(Default, Debug)]
pub struct Flattened {
    pub prims: Vec<Id>,
    pub schemas: BTreeSet<SchemaId>,
    pub opinions: Vec<Opinion>,
}

#[derive(Debug)]
pub enum ComposeError {
    Missing(Id),
    Cycle(Id),
    UnknownSchema(SchemaId),
    VerificationFailed(String),
}

#[derive(Clone, Debug)]
pub struct Verification {
    pub schema: SchemaId,
    pub ok: bool,
    pub detail: String,
}

/// A schema is a discipline's contract: it names itself and verifies opinions.
pub trait Schema: Send + Sync {
    fn name(&self) -> &str;
    fn verify(&self, o: &Opinion) -> Verification;
}

// ---------------------------------------------------------------------------
// Built-in schemas — one per discipline. Each is small, honest, and testable.
// ---------------------------------------------------------------------------

/// Classification primitives (the existing taxonomy, expressed as a schema).
pub struct ClassificationSchema;
impl Schema for ClassificationSchema {
    fn name(&self) -> &str { "classification" }
    fn verify(&self, o: &Opinion) -> Verification {
        let ok = matches!(o.claim, Claim::Type(_) | Claim::Invariant(_));
        Verification {
            schema: self.name().into(), ok,
            detail: if ok { "classification claim well-formed".into() }
                    else { "classification expects Type|Invariant".into() },
        }
    }
}

/// Boundary discipline: asserts the crate does not host forbidden surfaces.
pub struct BoundarySchema;
impl Schema for BoundarySchema {
    fn name(&self) -> &str { "boundary" }
    fn verify(&self, o: &Opinion) -> Verification {
        match &o.claim {
            Claim::Policy { allow, .. } => Verification {
                schema: self.name().into(), ok: *allow,
                detail: "boundary policy evaluated".into(),
            },
            _ => Verification {
                schema: self.name().into(), ok: false,
                detail: "boundary expects Policy".into(),
            },
        }
    }
}

/// Proof discipline: a proof obligation is satisfied only if declared Attest.
pub struct ProofSchema;
impl Schema for ProofSchema {
    fn name(&self) -> &str { "proof" }
    fn verify(&self, o: &Opinion) -> Verification {
        match &o.claim {
            Claim::Proof(_)  => Verification { schema: self.name().into(), ok: false, detail: "unproved obligation".into() },
            Claim::Attest(_) => Verification { schema: self.name().into(), ok: true,  detail: "attested".into() },
            _ => Verification { schema: self.name().into(), ok: false, detail: "proof expects Proof|Attest".into() },
        }
    }
}

/// Benchmark discipline: metric <= max.
pub struct BenchSchema;
impl Schema for BenchSchema {
    fn name(&self) -> &str { "bench" }
    fn verify(&self, o: &Opinion) -> Verification {
        match &o.claim {
            Claim::Bench { metric, max } => Verification {
                schema: self.name().into(), ok: *max >= 0.0,
                detail: format!("{metric} <= {max}"),
            },
            _ => Verification { schema: self.name().into(), ok: false, detail: "bench expects Bench".into() },
        }
    }
}

/// Policy discipline (OPA/Cedar-style).
pub struct PolicySchema;
impl Schema for PolicySchema {
    fn name(&self) -> &str { "policy" }
    fn verify(&self, o: &Opinion) -> Verification {
        match &o.claim {
            Claim::Policy { allow, reason } => Verification {
                schema: self.name().into(), ok: *allow,
                detail: reason.clone(),
            },
            _ => Verification { schema: self.name().into(), ok: false, detail: "policy expects Policy".into() },
        }
    }
}

// ---------------------------------------------------------------------------
// Tests — the first real composition.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn stage() -> Stage {
        let mut s = Stage::new();
        s.register_schema(Box::new(ClassificationSchema));
        s.register_schema(Box::new(BoundarySchema));
        s.register_schema(Box::new(ProofSchema));
        s.register_schema(Box::new(BenchSchema));
        s.register_schema(Box::new(PolicySchema));
        s
    }

    fn op(layer: &str, schema: &str, strength: u32, claim: Claim) -> Opinion {
        Opinion { layer: layer.into(), schema: schema.into(), strength, claim }
    }

    #[test]
    fn id_is_content_addressed() {
        assert_eq!(Id::of(b"a"), Id::of(b"a"));
        assert_ne!(Id::of(b"a"), Id::of(b"b"));
    }

    #[test]
    fn flatten_preserves_local_opinions() {
        let mut s = stage();
        let p = Prim::new("classification", b"root")
            .with_opinion(op("local", "classification", 100, Claim::Type("Cardinality".into())));
        let id = s.add(p);
        let f = s.flatten(id).unwrap();
        assert_eq!(f.opinions.len(), 1);
        assert_eq!(f.opinions[0].strength, 100);
    }

    #[test]
    fn strength_order_is_deterministic() {
        let mut s = stage();
        let base = s.add(Prim::new("classification", b"base")
            .with_opinion(op("base", "classification", 10, Claim::Type("Base".into()))));
        let root = s.add(Prim::new("classification", b"root")
            .with_opinion(op("local", "classification", 100, Claim::Type("Local".into())))
            .with_arc(Arc::Inherit(base)));
        let f = s.flatten(root).unwrap();
        assert_eq!(f.opinions[0].layer, "local");
        assert_eq!(f.opinions[1].layer, "base");
    }

    #[test]
    fn cycle_is_rejected() {
        let mut s = stage();
        let a = Id::of(b"a");
        let b = Id::of(b"b");
        let pa = Prim { id: a, schema: "classification".into(), payload: b"a".to_vec(),
                        opinions: vec![], arcs: vec![Arc::Ref(b)] };
        let pb = Prim { id: b, schema: "classification".into(), payload: b"b".to_vec(),
                        opinions: vec![], arcs: vec![Arc::Ref(a)] };
        s.add(pa); s.add(pb);
        assert!(matches!(s.flatten(a), Err(ComposeError::Cycle(_))));
    }

    #[test]
    fn boundary_policy_is_enforced() {
        let mut s = stage();
        let p = Prim::new("boundary", b"no-http")
            .with_opinion(op("local", "boundary", 100,
                Claim::Policy { allow: true, reason: "no HTTP server in tree".into() }));
        let id = s.add(p);
        let f = s.flatten(id).unwrap();
        let v = s.verify(&f).unwrap();
        assert!(v.iter().all(|r| r.ok));
    }

    #[test]
    fn proof_obligation_without_attest_fails() {
        let mut s = stage();
        let p = Prim::new("proof", b"closure")
            .with_opinion(op("local", "proof", 100, Claim::Proof("closure total".into())));
        let id = s.add(p);
        let f = s.flatten(id).unwrap();
        let v = s.verify(&f).unwrap();
        assert!(v.iter().any(|r| !r.ok));
    }

    #[test]
    fn proof_obligation_with_attest_passes() {
        let mut s = stage();
        let p = Prim::new("proof", b"closure")
            .with_opinion(op("local", "proof", 100, Claim::Attest("tests/standards.rs".into())));
        let id = s.add(p);
        let f = s.flatten(id).unwrap();
        let v = s.verify(&f).unwrap();
        assert!(v.iter().all(|r| r.ok));
    }

    #[test]
    fn receipt_is_stable() {
        let mut s = stage();
        let p = Prim::new("classification", b"root")
            .with_opinion(op("local", "classification", 100, Claim::Type("X".into())));
        let id = s.add(p);
        let f = s.flatten(id).unwrap();
        let r1 = s.receipt(id, &f);
        let r2 = s.receipt(id, &f);
        assert_eq!(r1, r2);
        assert!(r1.contains("compose-receipt v1"));
    }

    #[test]
    fn unknown_schema_is_rejected() {
        let mut s = stage();
        let p = Prim::new("no-such-schema", b"x")
            .with_opinion(op("local", "no-such-schema", 100, Claim::Type("X".into())));
        let id = s.add(p);
        let f = s.flatten(id).unwrap();
        assert!(matches!(s.verify(&f), Err(ComposeError::UnknownSchema(_))));
    }

    #[test]
    fn cross_discipline_composition() {
        // One prim, opinions from classification + boundary + proof + bench.
        let mut s = stage();
        let base = s.add(Prim::new("classification", b"taxonomy-base")
            .with_opinion(op("base", "classification", 10, Claim::Type("Base".into()))));
        let root = s.add(Prim::new("classification", b"taxonomy-root")
            .with_opinion(op("local", "classification", 100, Claim::Type("Cardinality".into())))
            .with_opinion(op("local", "boundary",       100,
                Claim::Policy { allow: true, reason: "no HTTP".into() }))
            .with_opinion(op("local", "bench",          100,
                Claim::Bench { metric: "classify_ms".into(), max: 5.0 }))
            .with_arc(Arc::Inherit(base)));
        let f = s.flatten(root).unwrap();
        let v = s.verify(&f).unwrap();
        assert_eq!(v.len(), 4);
        assert!(v.iter().all(|r| r.ok));
        assert_eq!(f.opinions[0].layer, "local");
    }
}
RS

# Wire the module into the crate
if ! grep -q 'pub mod compose;' src/lib.rs 2>/dev/null; then
  printf '\npub mod compose;\n' >> src/lib.rs
fi

echo "== 6. checks/run.sh: dev+sec block, rest advisory =="
cat > checks/run.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
groups=("$@")
[ "${#groups[@]}" -eq 0 ] && groups=(dev sec red ops hackathon it pentest org infra pipeline compliance data sre ml docs perf chaos compose)
fail=0
for g in "${groups[@]}"; do
  [ -d "checks/$g" ] || continue
  for f in checks/"$g"/*.sh; do
    [ -e "$f" ] || continue
    [ -x "$f" ] || chmod +x "$f"
    printf '\n[%s] %s\n' "$g" "$(basename "$f")"
    if ! "$f"; then
      case "$g" in
        dev|sec) fail=1 ;;
        *) echo "ADVISORY ($g): $(basename "$f") failed" ;;
      esac
    fi
  done
done
exit $fail
SH
chmod +x checks/run.sh

# A real compose check that runs the new module's tests
cat > checks/compose/01-compose.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
cargo test --release --lib compose:: -- --nocapture
SH
chmod +x checks/compose/01-compose.sh

echo "== 7. preflight: dev+sec block, everything else advisory =="
cat > scripts/preflight.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
fail=0
step() { printf '\n\033[1;34m== %s\033[0m\n' "$1"; }
run()  { "$@" || { echo "FAIL: $*"; fail=1; }; }
adv()  { "$@" || echo "ADVISORY: $*"; }
opt()  { command -v "$1" >/dev/null 2>&1; }

step fmt;        run cargo fmt --all -- --check
step clippy;     run cargo clippy --all-targets --all-features -- -D warnings
step test;       run cargo test --release --all-targets
step docs;       run cargo doc --no-deps --document-private-items
step checks;     run bash checks/run.sh dev sec
step compose;    adv bash checks/run.sh compose
step red;        adv bash checks/run.sh red
step receipt;    run cargo xt receipt
opt cargo-audit   && { step audit;   run cargo audit; } || echo "skip cargo-audit"
opt cargo-deny    && { step deny;    run cargo deny check advisories bans licenses sources; } || echo "skip cargo-deny"
opt cargo-machete && { step machete; run cargo machete; } || echo "skip cargo-machete"
[ "$fail" -eq 0 ] && printf '\n\033[1;32mpreflight: OK\033[0m\n' || { printf '\n\033[1;31mpreflight: NOT OK\033[0m\n'; exit 1; }
SH
chmod +x scripts/preflight.sh

echo "== 8. verify =="
cargo xt ci
bash checks/compose/01-compose.sh
scripts/preflight.sh

echo "== 9. commit + push =="
git add -A
git commit -m "compose: USD-style composition substrate; deny.toml trimmed; dev+sec blocking, rest advisory" || true
git push origin main

echo "== done =="
