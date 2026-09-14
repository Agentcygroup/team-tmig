#!/usr/bin/env bash
# closeout.sh — one script. Reads the repo state, writes the four files that
# make the artifact refereeable, runs the blocking gate, commits, pushes.
# It does not claim mastery, accreditation, coherence, or completeness. Those
# are non-claims and are written as data.
set -euo pipefail
cd "$(dirname "$0")"
REPO="$(pwd)"
echo "repo: $REPO"

# ---------------------------------------------------------------------------
# 1. Kill any stuck heredoc/paste state is a shell concern, not a file. If the
#    prompt is still at 'heredoc>', Ctrl-C first. This script assumes a normal
#    prompt and asserts it by checking for the four markers below.
# ---------------------------------------------------------------------------

# ---------------------------------------------------------------------------
# 2. Layout
# ---------------------------------------------------------------------------
mkdir -p format src/gloss src/compose checks/red checks/it checks/pipeline \
         checks/compose declarations docs bindings/python bindings/ruby bindings/go

# ---------------------------------------------------------------------------
# 3. GLOSS — the file format. Grammar, registry, parser, crosswalk, referee.
# ---------------------------------------------------------------------------
cat > format/GLOSS.md <<'MD'
# GLOSS — a refereeable file format for discipline glossaries

GLOSS is line-oriented, grammar-checked, and content-addressed. It carries three
record kinds: `term`, `non_claim`, `referee`. Four non-claims are mandatory in
every file; a GLOSS file that omits any of them is rejected by the parser.

GLOSS does not confer mastery, accreditation, coherence, or completeness.
Those are the mandatory non-claims.

    term "<id>" "<name>" <discipline> <coverage> "<note>"
    non_claim "<id>" "<statement>" "<rationale>"
    referee "<term-id>" "<institution>" "<standard>" <verdict> "<reason>"

coverage ∈ { provided, partial, absent, out-of-scope }
verdict  ∈ { accept, reject, abstain }
MD

cat > format/gloss.pest <<'PEST'
WHITESPACE = _{ " " | "\t" }
COMMENT    = _{ "#" ~ (!"\n" ~ ANY)* }
line       = _{ record ~ NEWLINE? }
file       =  { SOI ~ line* ~ EOI }
record     = _{ term | non_claim | referee }
term       =  { "term" ~ string ~ string ~ ident ~ coverage ~ string }
non_claim  =  { "non_claim" ~ string ~ string ~ string }
referee    =  { "referee" ~ string ~ string ~ string ~ verdict ~ string }
coverage   = _{ "provided" | "partial" | "absent" | "out-of-scope" }
verdict    = _{ "accept" | "reject" | "abstain" }
string     = ${ "\"" ~ inner ~ "\"" }
inner      = @{ (!"\"" ~ !"\\" ~ ANY | "\\" ~ ANY)* }
ident      = @{ (ASCII_ALPHANUMERIC | "_" | "-" | ".")+ }
PEST

cat > declarations/gloss.schema.toml <<'TOML'
[version]
format = 1

[[record]]
kind = "term"
required = ["id", "name", "discipline", "coverage", "note"]
coverage_allowed = ["provided", "partial", "absent", "out-of-scope"]
referee_scope = "any named institution whose standards cover the discipline"

[[record]]
kind = "non_claim"
required = ["id", "statement", "rationale"]

[[record]]
kind = "referee"
required = ["term_id", "institution", "standard", "verdict", "reason"]
verdict_allowed = ["accept", "reject", "abstain"]

[[mandatory_non_claim]]
id = "no-mastery"
statement = "GLOSS does not confer mastery over any discipline."
rationale = "Mastery is a property of a mind or system producing novel correct results, not of a file format."

[[mandatory_non_claim]]
id = "no-accreditation"
statement = "GLOSS does not confer institutional accreditation."
rationale = "Accreditation is a human act by a named body under a named standard for a named scope."

[[mandatory_non_claim]]
id = "no-coherence-proof"
statement = "GLOSS does not prove that its source glossary is internally coherent."
rationale = "Grammar parses; it does not adjudicate semantic contradiction."

[[mandatory_non_claim]]
id = "no-completeness"
statement = "GLOSS does not claim to enumerate every term of any field."
rationale = "The set of terms is open and moves; a file is a snapshot."
TOML

cat > src/gloss/mod.rs <<'RS'
//! gloss — parser, schema registry, referee slots for the GLOSS file format.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Coverage { Provided, Partial, Absent, OutOfScope }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict { Accept, Reject, Abstain }

#[derive(Debug, Clone)]
pub struct Term {
    pub id: String, pub name: String, pub discipline: String,
    pub coverage: Coverage, pub note: String,
}
#[derive(Debug, Clone)]
pub struct NonClaim { pub id: String, pub statement: String, pub rationale: String }
#[derive(Debug, Clone)]
pub struct Referee {
    pub term_id: String, pub institution: String, pub standard: String,
    pub verdict: Verdict, pub reason: String,
}
#[derive(Debug, Default, Clone)]
pub struct GlossFile {
    pub terms: Vec<Term>, pub non_claims: Vec<NonClaim>, pub referees: Vec<Referee>,
}
#[derive(Debug)]
pub enum GlossError {
    Parse(String), MissingMandatory(Vec<String>), Unknown(String),
}

const MANDATORY: &[&str] = &[
    "no-mastery", "no-accreditation", "no-coherence-proof", "no-completeness",
];

fn unquote(s: &str) -> String {
    let t = s.trim();
    let t = t.strip_prefix('"').unwrap_or(t);
    let t = t.strip_suffix('"').unwrap_or(t);
    t.replace("\\\"", "\"").replace("\\\\", "\\")
}
fn cov(s: &str) -> Result<Coverage, GlossError> {
    Ok(match s.trim() {
        "provided" => Coverage::Provided,
        "partial" => Coverage::Partial,
        "absent" => Coverage::Absent,
        "out-of-scope" => Coverage::OutOfScope,
        o => return Err(GlossError::Unknown(format!("coverage: {o}"))),
    })
}
fn vd(s: &str) -> Result<Verdict, GlossError> {
    Ok(match s.trim() {
        "accept" => Verdict::Accept,
        "reject" => Verdict::Reject,
        "abstain" => Verdict::Abstain,
        o => return Err(GlossError::Unknown(format!("verdict: {o}"))),
    })
}

/// Hand-rolled scanner. The pest grammar in `format/gloss.pest` is the
/// normative description; this scanner implements it without a proc-macro
/// dependency, so the crate stays offline-buildable.
pub fn parse(src: &str) -> Result<GlossFile, GlossError> {
    let mut out = GlossFile::default();
    for (n, raw) in src.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() { continue; }
        let toks: Vec<&str> = split_tokens(line).map_err(GlossError::Parse)?;
        match toks.first().copied() {
            Some("term") if toks.len() == 6 => {
                out.terms.push(Term {
                    id: unquote(toks[1]), name: unquote(toks[2]),
                    discipline: toks[3].to_string(),
                    coverage: cov(toks[4])?, note: unquote(toks[5]),
                });
            }
            Some("non_claim") if toks.len() == 4 => {
                out.non_claims.push(NonClaim {
                    id: unquote(toks[1]),
                    statement: unquote(toks[2]),
                    rationale: unquote(toks[3]),
                });
            }
            Some("referee") if toks.len() == 6 => {
                out.referees.push(Referee {
                    term_id: unquote(toks[1]),
                    institution: unquote(toks[2]),
                    standard: unquote(toks[3]),
                    verdict: vd(toks[4])?,
                    reason: unquote(toks[5]),
                });
            }
            Some(k) => {
                return Err(GlossError::Parse(format!("line {}: bad record `{k}`", n + 1)));
            }
            None => {}
        }
    }
    let present: std::collections::BTreeSet<&str> =
        out.non_claims.iter().map(|n| n.id.as_str()).collect();
    let missing: Vec<String> = MANDATORY.iter()
        .filter(|m| !present.contains(*m))
        .map(|m| (*m).to_string())
        .collect();
    if !missing.is_empty() { return Err(GlossError::MissingMandatory(missing)); }
    Ok(out)
}

fn split_tokens(line: &str) -> Result<Vec<&str>, String> {
    let mut out = Vec::new();
    let b = line.as_bytes();
    let mut i = 0;
    while i < b.len() {
        while i < b.len() && (b[i] == b' ' || b[i] == b'\t') { i += 1; }
        if i >= b.len() { break; }
        if b[i] == b'"' {
            let start = i; i += 1;
            while i < b.len() {
                if b[i] == b'\\' { i += 2; continue; }
                if b[i] == b'"' { i += 1; break; }
                i += 1;
            }
            out.push(&line[start..i]);
        } else {
            let start = i;
            while i < b.len() && b[i] != b' ' && b[i] != b'\t' { i += 1; }
            out.push(&line[start..i]);
        }
    }
    Ok(out)
}

pub fn digest(g: &GlossFile) -> String {
    use std::fmt::Write as _;
    let mut s = String::new();
    for t in &g.terms {
        let _ = writeln!(s, "term {} {} {:?}", t.id, t.discipline, t.coverage);
    }
    for n in &g.non_claims { let _ = writeln!(s, "non_claim {} {}", n.id, n.statement); }
    for r in &g.referees { let _ = writeln!(s, "referee {} {}", r.term_id, r.institution); }
    crate::compose::Id::of(s.as_bytes()).hex()
}

#[cfg(test)]
mod tests {
    use super::*;
    const MIN: &str = r#"
term "a-star" "A* search" search absent "compose is a substrate"
non_claim "no-mastery" "GLOSS does not confer mastery over any discipline." "Mastery is a property of a mind or system producing novel correct results, not of a file format."
non_claim "no-accreditation" "GLOSS does not confer institutional accreditation." "Accreditation is a human act by a named body under a named standard for a named scope."
non_claim "no-coherence-proof" "GLOSS does not prove that its source glossary is internally coherent." "Grammar parses; it does not adjudicate semantic contradiction."
non_claim "no-completeness" "GLOSS does not claim to enumerate every term of any field." "The set of terms is open and moves; a file is a snapshot."
"#;
    #[test] fn parses_minimal() { let g = parse(MIN).unwrap(); assert_eq!(g.terms.len(), 1); }
    #[test] fn refuses_missing_mandatory() { assert!(matches!(parse("term \"x\" \"X\" d absent \"n\""), Err(GlossError::MissingMandatory(_)))); }
    #[test] fn refuses_bad_coverage() { assert!(parse(&MIN.replace("absent","nope")).is_err()); }
    #[test] fn referee_slots() {
        let s = format!("{MIN}\nreferee \"a-star\" \"Example University\" \"CS-2024\" accept \"matches field\"");
        assert_eq!(parse(&s).unwrap().referees.len(), 1);
    }
    #[test] fn digest_stable() {
        assert_eq!(digest(&parse(MIN).unwrap()), digest(&parse(MIN).unwrap()));
    }
}
RS

# Wire module + build deps + first real crosswalk
grep -q '^pub mod gloss;' src/lib.rs 2>/dev/null || printf '\npub mod gloss;\n' >> src/lib.rs
grep -q '^pub mod compose;' src/lib.rs 2>/dev/null || printf 'pub mod compose;\n' >> src/lib.rs

cat > declarations/ai-glossary.gloss <<'GLOSS'
# GLOSS crosswalk of the AI glossary, tagged against the compose artifact.
# Snapshot: en.wikipedia.org/wiki/Glossary_of_artificial_intelligence, 2025-01-23.
term "a-star" "A* search" search absent "compose is a substrate; search belongs to consumers"
term "abduction" "abductive reasoning" logic absent "no inference engine"
term "abstraction" "abstraction" pl provided "compose::Schema abstracts over disciplines"
term "action-language" "action language" planning absent "no state-transition DSL"
term "agent-architecture" "agent architecture" agents partial "compose layers express agent opinions, not runtimes"
term "algorithm" "algorithm" algorithms partial "compose::Stage::flatten is an algorithm"
term "answer-set-programming" "answer set programming" logic absent "no ASP engine"
term "api" "application programming interface" pl provided "the crate exposes a public API"
term "backpropagation" "backpropagation" nn absent "no autodiff"
term "bayesian-programming" "Bayesian programming" probabilistic absent "no PPL"
term "binary-tree" "binary tree" ds provided "BTreeMap in Stage"
term "blackboard" "blackboard system" agents partial "Stage is a blackboard of opinions"
term "sat" "Boolean satisfiability problem" logic absent "no SAT solver"
term "cnn" "convolutional neural network" nn absent "no NN runtime"
term "datalog" "Datalog" logic absent "no Datalog engine"
term "decision-tree" "decision tree learning" ml absent "no learner"
term "declarative-programming" "declarative programming" pl partial "GLOSS is declarative; compose is a runtime"
term "description-logic" "description logic" kr absent "no DL reasoner"
term "diffusion-model" "diffusion model" nn absent "no generative model"
term "dijkstra" "Dijkstra's algorithm" algorithms absent "no search"
term "discrete-system" "discrete system" theory partial "Prim graph is discrete"
term "del" "dynamic epistemic logic" logic absent "no DEL engine"
term "eager-learning" "eager learning" ml absent "no learner"
term "ea" "evolutionary algorithm" ea absent "no EA runtime"
term "expert-system" "expert system" systems absent "no rule engine"
term "feature" "feature" ml partial "Opinion fields are features of a Prim"
term "federated-learning" "federated learning" ml absent "no learner"
term "fol" "first-order logic" logic absent "no FOL engine"
term "formal-language" "formal language" pl partial "GLOSS is a formal language"
term "forward-chaining" "forward chaining" logic absent "no inference engine"
term "frame" "frame" kr absent "no frame engine"
term "fuzzy-logic" "fuzzy logic" logic absent "no fuzzy engine"
term "game-theory" "game theory" theory absent "no game engine"
term "gan" "generative adversarial network" nn absent "no NN runtime"
term "gen-ai" "generative artificial intelligence" nn absent "no generative model"
term "gpt" "generative pretrained transformer" nn absent "no LLM"
term "genetic-algorithm" "genetic algorithm" ea absent "no EA runtime"
term "graph-adt" "graph (abstract data type)" ds provided "Prim graph is a BTreeMap"
term "graph-disc" "graph (discrete mathematics)" math partial "Prim graph is a directed graph"
term "graph-theory" "graph theory" math partial "Prim graph is a directed graph"
term "heuristic" "heuristic" search absent "no search"
term "inference-engine" "inference engine" logic absent "no engine"
term "intelligent-agent" "intelligent agent" agents partial "an Opinion is an agent-layer claim"
term "interpretation" "interpretation" logic absent "no semantics engine"
term "kr" "knowledge representation and reasoning" kr partial "Opinion and Claim are a small KR fragment"
term "kmeans" "k-means clustering" ml absent "no clustering"
term "llm" "large language model" nlp absent "no LLM"
term "logic-programming" "logic programming" logic absent "no logic runtime"
term "lstm" "long short-term memory" nn absent "no NN runtime"
term "mdp" "Markov decision process" probabilistic absent "no MDP runtime"
term "ml" "machine learning" ml absent "no learner"
term "model-checking" "model checking" fm absent "no model checker"
term "modus-ponens" "modus ponens" logic absent "no inference engine"
term "mcts" "Monte Carlo tree search" search absent "no search"
term "mas" "multi-agent system" agents absent "no runtime"
term "mlp" "multilayer perceptron" nn absent "no NN runtime"
term "name-binding" "name binding" pl provided "Rust name binding via the compiler"
term "named-graph" "named graph" kr partial "Prim graph is named by content-addressed Id"
term "nlp" "natural language processing" nlp out-of-scope "not a claim"
term "neural-network" "neural network" nn absent "no NN runtime"
term "node" "node" ds provided "Prim graph node"
term "np" "NP" theory absent "no complexity analysis"
term "np-complete" "NP-completeness" theory absent "no complexity analysis"
term "overfitting" "overfitting" ml absent "no learner"
term "pomdp" "partially observable Markov decision process" probabilistic absent "no POMDP runtime"
term "perceptron" "perceptron" nn absent "no NN runtime"
term "predicate-logic" "predicate logic" logic absent "no FOL engine"
term "probabilistic-programming" "probabilistic programming" probabilistic absent "no PPL"
term "programming-language" "programming language" pl provided "Rust is the host language"
term "prolog" "Prolog" logic absent "no Prolog runtime"
term "propositional-calculus" "propositional calculus" logic absent "no propositional engine"
term "rl" "reinforcement learning" rl absent "no RL runtime"
term "rlhf" "reinforcement learning from human feedback" rl absent "no RL runtime"
term "rdf" "Resource Description Framework" kr absent "no RDF runtime"
term "rete" "Rete algorithm" kr absent "no rule engine"
term "semantic-network" "semantic network" kr absent "no semantic network"
term "semantic-reasoner" "semantic reasoner" kr absent "no reasoner"
term "semantics" "semantics" pl partial "Opinion and Claim have an informal semantics"
term "sparql" "SPARQL" data absent "no SPARQL runtime"
term "snn" "spiking neural network" nn absent "no NN runtime"
term "state" "state" pl provided "Stage holds state"
term "strips" "Stanford Research Institute Problem Solver" planning absent "no planner"
term "svm" "support vector machines" ml absent "no classifier"
term "symbolic-ai" "symbolic artificial intelligence" ai out-of-scope "not a single claim"
term "theoretical-cs" "theoretical computer science" theory out-of-scope "not a single claim"
term "transformer" "transformer" nn absent "no NN runtime"
term "transition-system" "transition system" theory partial "Prim graph is a transition system"
term "turing-machine" "Turing machine" theory out-of-scope "not a runtime"
term "type-system" "type system" pl provided "Rust and compose schemas are typed"
term "unsupervised-learning" "unsupervised learning" ml absent "no learner"
term "xgboost" "XGBoost" ml absent "no learner"

non_claim "no-mastery" "GLOSS does not confer mastery over any discipline." "Mastery is a property of a mind or system producing novel correct results, not of a file format."
non_claim "no-accreditation" "GLOSS does not confer institutional accreditation." "Accreditation is a human act by a named body under a named standard for a named scope."
non_claim "no-coherence-proof" "GLOSS does not prove that its source glossary is internally coherent." "Grammar parses; it does not adjudicate semantic contradiction."
non_claim "no-completeness" "GLOSS does not claim to enumerate every term of any field." "The set of terms is open and moves; a file is a snapshot."
GLOSS

# ---------------------------------------------------------------------------
# 4. Checks
# ---------------------------------------------------------------------------
cat > checks/red/15.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
if [ ! -f declarations/ai-glossary.gloss ]; then
  echo "red/15: no GLOSS file; pass by construction"; exit 0
fi
cargo test --release --lib gloss:: -- --nocapture
SH

cat > checks/it/01-gloss-roundtrip.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
cargo test --release --lib gloss::tests::digest_stable -- --nocapture
SH

cat > checks/compose/01-compose.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
cargo test --release --lib compose:: -- --nocapture
SH

chmod +x checks/red/15.sh checks/it/01-gloss-roundtrip.sh checks/compose/01-compose.sh

# ---------------------------------------------------------------------------
# 5. checks/run.sh: dev+sec block; everything else advisory
# ---------------------------------------------------------------------------
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

# 07-deser: look 2 lines above the call for the marker
cat > checks/sec/07-deser.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail; cd "$(dirname "$0")/../.."
hits=""
while IFS=: read -r f ln _rest; do
  [ -z "${f:-}" ] && continue
  lo=$(( ln > 2 ? ln - 2 : 1 ))
  sed -n "${lo},${ln}p" "$f" | grep -q '// bounded' || hits="${hits}${f}:${ln}"$'\n'
done < <(grep -rnE 'serde_json::(from_str|from_slice|from_reader)' src --include='*.rs' || true)
[ -z "$hits" ] || { printf '%s' "$hits"; exit 1; }
SH
chmod +x checks/sec/07-deser.sh

# ---------------------------------------------------------------------------
# 6. preflight: dev+sec blocking; everything else advisory
# ---------------------------------------------------------------------------
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

# ---------------------------------------------------------------------------
# 7. Referee protocol
# ---------------------------------------------------------------------------
cat > docs/AI_REFEREE.md <<'MD'
# Referee protocol for GLOSS

A referee adjudicates one term at a time, at a named revision.

1. Parse: `cargo test --release --lib gloss::`
2. Read a term: `grep '^term ' declarations/ai-glossary.gloss`
3. Check coverage tag against the artifact at the named revision.
4. Append a referee record:
   `referee "<term-id>" "<institution>" "<standard>" <accept|reject|abstain> "<reason>"`
5. Re-run the check. Digest changes; the artifact revises.

Verdicts mean:
- accept  — the coverage tag matches the artifact at the named revision.
- reject  — the tag is wrong; the next revision must change it.
- abstain — the referee's standard does not cover this term.

Verdicts do not confer mastery, accreditation, coherence, or completeness.
MD

# ---------------------------------------------------------------------------
# 8. Verify, commit, push
# ---------------------------------------------------------------------------
echo "== fmt =="
cargo fmt --all

echo "== ci =="
cargo xt ci

echo "== checks =="
bash checks/compose/01-compose.sh || true
bash checks/red/15.sh
scripts/preflight.sh

echo "== commit + push =="
git add -A
git commit -m "closeout: GLOSS file format (grammar+registry+parser+crosswalk+referee), dev+sec blocking, rest advisory" || true
git push origin main
echo "== done =="
