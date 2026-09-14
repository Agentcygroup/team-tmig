//! The normalizer: surface form -> canonical tag | reject.
//!
//! The function is deterministic and total over the 11 canonical tags.
//! Surface forms that fail P (no content-addressed terminal state) or S
//! (not a substrate concern) are rejected with a named reason.

use crate::tags::canonical::Canonical;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormResult {
    Canonical(Canonical),
    Rejected(RejectReason),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RejectReason {
    /// Fails P: no content-addressed terminal state.
    NoTerminalState,
    /// Fails S: not a substrate concern.
    NotSubstrate,
}

pub fn normalize(surface: &str) -> NormResult {
    // Strip trailing '!' (breaking-change marker) and any (scope) suffix.
    let s = surface.trim().to_ascii_lowercase();
    let s = s
        .split('(')
        .next()
        .unwrap_or(&s)
        .trim_end_matches('!')
        .trim();

    match s {
        // feat
        "feat" | "feature" | "new" | "add" | "implement" | "introduce" | "support" => {
            NormResult::Canonical(Canonical::Feat)
        }
        // fix
        "fix" | "bugfix" | "hotfix" | "bug" | "patch" | "repair" | "correct" => {
            NormResult::Canonical(Canonical::Fix)
        }
        // refactor
        "refactor" | "refact" | "restructure" | "reorganize" | "cleanup" | "clean" | "simplify"
        | "reduce" | "dedup" | "deduplicate" => NormResult::Canonical(Canonical::Refactor),
        // perf
        "perf" | "performance" | "optimize" | "optimise" | "tune" | "speedup" | "faster"
        | "cache" | "lazy" => NormResult::Canonical(Canonical::Perf),
        // docs
        "docs" | "doc" | "documentation" | "comment" | "readme" | "changelog" | "manual" => {
            NormResult::Canonical(Canonical::Docs)
        }
        // test
        "test" | "tests" | "spec" | "specs" | "coverage" | "assert" | "verify" | "fixture"
        | "mock" => NormResult::Canonical(Canonical::Test),
        // build
        "build" | "builds" | "compile" | "compile-time" | "link" | "linker" | "toolchain"
        | "bootstrap" => NormResult::Canonical(Canonical::Build),
        // ci
        "ci" | "travis" | "circle" | "github-actions" | "jenkins" | "gitlab-ci" | "pipeline"
        | "workflow" => NormResult::Canonical(Canonical::Ci),
        // chore
        "chore" | "misc" | "housekeeping" | "maintenance" | "tidy" | "whitespace" | "typo"
        | "lint" | "format" | "style" => NormResult::Canonical(Canonical::Chore),
        // sec
        "sec" | "security" | "cve" | "vuln" | "vulnerability" | "hardening" | "crypto" | "auth"
        | "permission" => NormResult::Canonical(Canonical::Sec),
        // revert
        "revert" | "rollback" | "undo" | "restore" | "backout" => {
            NormResult::Canonical(Canonical::Revert)
        }
        // out-of-scope: fails P
        "wip" | "experiment" | "spike" | "poc" | "proto" | "nit" => {
            NormResult::Rejected(RejectReason::NoTerminalState)
        }
        // out-of-scope: fails S
        "release" | "merge" | "cherry-pick" | "squash" | "rebase" | "breaking-change"
        | "skip-ci" => NormResult::Rejected(RejectReason::NotSubstrate),
        _ => NormResult::Rejected(RejectReason::NotSubstrate),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_canonical() {
        assert_eq!(normalize("feat"), NormResult::Canonical(Canonical::Feat));
        assert_eq!(normalize("feature"), NormResult::Canonical(Canonical::Feat));
        assert_eq!(
            normalize("feat(scope)"),
            NormResult::Canonical(Canonical::Feat)
        );
        assert_eq!(normalize("feat!"), NormResult::Canonical(Canonical::Feat));
    }

    #[test]
    fn rejects_wip() {
        assert_eq!(
            normalize("wip"),
            NormResult::Rejected(RejectReason::NoTerminalState)
        );
    }

    #[test]
    fn rejects_release() {
        assert_eq!(
            normalize("release"),
            NormResult::Rejected(RejectReason::NotSubstrate)
        );
    }

    #[test]
    fn normalizes_all_canonical_names() {
        for c in crate::tags::canonical::CANONICAL_TAGS {
            let r = normalize(c.name());
            assert_eq!(
                r,
                NormResult::Canonical(c),
                "{} did not normalize",
                c.name()
            );
        }
    }

    #[test]
    fn is_total_on_unknown() {
        // unknown input -> reject with a named reason, never panic
        let r = normalize("this-is-not-a-tag");
        assert!(matches!(r, NormResult::Rejected(_)));
    }
}
