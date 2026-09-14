#!/usr/bin/env bash
# realign.sh — one-shot: fix .gitignore, untrack derived data, add tooling, wire xt, verify.
set -euo pipefail
cd "$(dirname "$0")"

say() { printf '\n\033[1;34m== %s\033[0m\n' "$*"; }
ok()  { printf '  \033[32m✓\033[0m %s\n' "$*"; }
bad() { printf '  \033[31m✗\033[0m %s\n' "$*"; }

# ---------------------------------------------------------------------------
say "1. Write canonical config files"
touch .gitignore
for pat in '/target/' '**/*.rs.bk' '*.bak' '*.bak.*' 'build.err' 'test.err' \
           '.DS_Store' '.audit/objects/' 'data/tiger/*.zip' 'data/tiger/*.txt' \
           'fuzz/target/' 'fuzz/Cargo.lock'
do
  grep -qxF "$pat" .gitignore || echo "$pat" >> .gitignore
done
ok ".gitignore updated"

mkdir -p .cargo .github/workflows scripts src/bin checks

cat > .cargo/config.toml <<'TOML'
[alias]
xt = "run --quiet --bin xt --"
TOML
ok ".cargo/config.toml"

cat > rust-toolchain.toml <<'TOML'
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
profile = "minimal"
TOML
ok "rust-toolchain.toml"

cat > rustfmt.toml <<'TOML'
edition = "2021"
max_width = 100
use_field_init_shorthand = true
TOML
ok "rustfmt.toml"

cat > clippy.toml <<'TOML'
cognitive-complexity-threshold = 30
too-many-arguments-threshold = 10
TOML
ok "clippy.toml"

cat > deny.toml <<'TOML'
[advisories]
vulnerability = "deny"
unmaintained = "warn"

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-3-Clause", "ISC", "Unicode-DFS-2016", "CC0-1.0"]

[bans]
multiple-versions = "warn"
TOML
ok "deny.toml"

cat > .editorconfig <<'EDITOR'
root = true
[*]
end_of_line = lf
insert_final_newline = true
charset = utf-8
indent_style = space
indent_size = 2
[*.rs]
indent_size = 4
EDITOR
ok ".editorconfig"

cat > .gitattributes <<'GA'
* text=auto eol=lf
*.rs text eol=lf
*.toml text eol=lf
*.lock text eol=lf
*.zip binary
*.jar binary
*.png binary
GA
ok ".gitattributes"

cat > .github/workflows/ci.yml <<'YML'
name: ci
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with: { components: rustfmt, clippy }
      - run: cargo fmt --check
      - run: cargo clippy --all-targets -- -D warnings
      - run: cargo test --release --all-targets
YML
ok ".github/workflows/ci.yml"

# ---------------------------------------------------------------------------
say "2. Write xt.rs and checks/run.sh and scripts/receipt.sh"

if [ ! -f src/bin/xt.rs ]; then
  cat > src/bin/xt.rs <<'RUST'
//! cargo xt — task runner aligned with CI.
use std::process::{exit, Command};

fn run(cmd: &str, args: &[&str]) -> bool {
    eprintln!("\n$ {} {}", cmd, args.join(" "));
    Command::new(cmd)
        .args(args)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn main() {
    let task = std::env::args().nth(1).unwrap_or_else(|| "help".into());
    let ok = match task.as_str() {
        "ci" => {
            run("cargo", &["fmt", "--check"])
                && run("cargo", &["clippy", "--all-targets", "--", "-D", "warnings"])
                && run("cargo", &["test", "--release", "--all-targets"])
        }
        "fmt"     => run("cargo", &["fmt"]),
        "fmt-check" => run("cargo", &["fmt", "--check"]),
        "lint"    => run("cargo", &["clippy", "--all-targets", "--", "-D", "warnings"]),
        "test"    => run("cargo", &["test", "--release", "--all-targets"]),
        "build"   => run("cargo", &["build", "--release"]),
        "receipt" => {
            std::fs::create_dir_all("target/provenance").ok();
            let out = std::process::Command::new("git")
                .args(["rev-parse", "HEAD"])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .unwrap_or_default();
            let body = format!("green baseline\ncommit: {}\n", out.trim());
            std::fs::write("target/provenance/green-receipt.txt", &body).is_ok()
        }
        "clean" => run("cargo", &["clean"]),
        _ => {
            eprintln!("usage: cargo xt <ci|fmt|fmt-check|lint|test|build|receipt|clean>");
            false
        }
    };
    exit(if ok { 0 } else { 1 });
}
RUST
  ok "src/bin/xt.rs"
else
  ok "src/bin/xt.rs already exists"
fi

if [ ! -f checks/run.sh ]; then
  cat > checks/run.sh <<'SH'
#!/usr/bin/env bash
# Unified runner for everything under checks/.
set -euo pipefail
cd "$(dirname "$0")/.."
groups=("$@")
[ "${#groups[@]}" -eq 0 ] && groups=(dev sec red ops hackathon)
fail=0
for g in "${groups[@]}"; do
  [ -d "checks/$g" ] || continue
  for f in checks/"$g"/*.sh; do
    [ -x "$f" ] || chmod +x "$f"
    printf '\n[%s] %s\n' "$g" "$(basename "$f")"
    if ! "$f"; then fail=1; fi
  done
done
exit $fail
SH
  chmod +x checks/run.sh
  ok "checks/run.sh"
else
  chmod +x checks/run.sh
  ok "checks/run.sh already exists"
fi

if [ ! -f scripts/receipt.sh ]; then
  cat > scripts/receipt.sh <<'SH'
#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p target/provenance
sha=$(git rev-parse HEAD)
{
  echo "green baseline"
  echo "commit: $sha"
  echo "date:   $(date -u +%Y-%m-%dT%H:%M:%SZ)"
} > target/provenance/green-receipt.txt
echo "target/provenance/green-receipt.txt"
SH
  chmod +x scripts/receipt.sh
  ok "scripts/receipt.sh"
else
  chmod +x scripts/receipt.sh
  ok "scripts/receipt.sh already exists"
fi

if [ ! -f scripts/fetch-tiger.sh ]; then
  cat > scripts/fetch-tiger.sh <<'SH'
#!/usr/bin/env bash
# Reproducible TIGER/Line fetch (2023 tract shapefiles per state FIPS).
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p data/tiger
states=(01 02 04 05 06 08 09 10 11 12 13 15 16 17 18 19 20 21 22 23 24 25 26 27
        28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 44 45 46 47 48 49 50 51
        53 54 55 56 60 66 69 72 78)
for s in "${states[@]}"; do
  url="https://www2.census.gov/geo/tiger/TIGER2023/TRACT/tl_2023_${s}_tract.zip"
  out="data/tiger/tl_2023_${s}_tract.zip"
  [ -f "$out" ] && { echo "have $out"; continue; }
  echo "fetch $url"
  curl -fL --retry 3 -o "$out.tmp" "$url"
  mv "$out.tmp" "$out"
done
echo "done"
SH
  chmod +x scripts/fetch-tiger.sh
  ok "scripts/fetch-tiger.sh"
else
  chmod +x scripts/fetch-tiger.sh
  ok "scripts/fetch-tiger.sh already exists"
fi

# ---------------------------------------------------------------------------
say "3. Untrack derived + machine-local files"
untrack() {
  local n
  n=$(git ls-files -- "$@" | wc -l | tr -d ' ')
  if [ "$n" -gt 0 ]; then
    git rm -r --cached --quiet -- "$@" || true
    ok "untracked $n path(s): $*"
  else
    ok "nothing tracked: $*"
  fi
}
untrack target
untrack 'data/tiger/*.zip'
untrack 'data/tiger/*.txt'
untrack 'fuzz/target'
untrack '*.bak'
untrack '*.bak.*'
untrack 'bin/fix-round-*.sh'
untrack 'bin/bootstrap-fix-all.sh'
untrack 'bin/clippy-fix.sh'
untrack 'bin/fix-thesis-2.sh'
untrack 'bin/factory-baseline.sh'

# ---------------------------------------------------------------------------
say "4. Stage tooling + any remaining changes"
to_stage=()
for p in .gitignore .gitattributes .editorconfig .cargo .github \
         rust-toolchain.toml rustfmt.toml clippy.toml deny.toml \
         scripts checks src/bin/xt.rs CONTRIBUTING.md CHANGELOG.md
do
  [ -e "$p" ] && to_stage+=("$p")
done
git add -A
ok "staged ${#to_stage[@]} tooling path(s) + working tree"

if ! git diff --cached --quiet; then
  git commit -m "chore: realign tracked set — ignore derived data, add tooling, wire cargo xt"
  ok "committed"
else
  ok "nothing to commit"
fi

# ---------------------------------------------------------------------------
say "5. Verify"
fail=0
z() { local n; n=$(git ls-files | grep -cE "$2" || true); if [ "$n" -eq 0 ]; then ok "$1: 0"; else bad "$1: $n"; fail=1; fi; }
p() { local n; n=$(git ls-files | grep -cE "$2" || true); if [ "$n" -ge 1 ]; then ok "$1: $n"; else bad "$1: 0 (want ≥1)"; fail=1; fi; }

echo "  -- must be 0 --"
z "target/"          '^target/'
z ".bak files"       '\.bak(\.|$)'
z "fuzz/target/"     '^fuzz/target/'
z "tiger zips"       '^data/tiger/.*\.zip$'
z ".audit/objects/"  '^\.audit/objects/'
z "bin/fix-*"        '^bin/fix-'

echo "  -- must be ≥1 --"
p ".audit/log/"      '^\.audit/log/'
p "scripts/"         '^scripts/'
p "src/bin/xt.rs"    '^src/bin/xt\.rs$'
p "checks/run.sh"    '^checks/run\.sh$'

echo
printf 'total tracked files: %s\n' "$(git ls-files | wc -l | tr -d ' ')"

if ! git rev-parse -q --verify refs/tags/v0.1.0-green >/dev/null; then
  git tag -a v0.1.0-green -m "green baseline"
  ok "tagged v0.1.0-green"
fi

echo
if [ "$fail" -eq 0 ]; then
  printf '\033[1;32mrealign: OK — now run: cargo xt ci\033[0m\n'
else
  printf '\033[1;31mrealign: NOT OK — see ✗ above\033[0m\n'
  exit 1
fi
