//! The 11 canonical tags ever used. Each is a fixed sequence over the 36
//! generative tags. Nothing else is canonical.

use crate::tags::vocabulary::{Concern, Mode, Tag};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Canonical {
    Feat,
    Fix,
    Refactor,
    Perf,
    Docs,
    Test,
    Build,
    Ci,
    Chore,
    Sec,
    Revert,
}

impl Canonical {
    pub const fn name(self) -> &'static str {
        match self {
            Canonical::Feat => "feat",
            Canonical::Fix => "fix",
            Canonical::Refactor => "refactor",
            Canonical::Perf => "perf",
            Canonical::Docs => "docs",
            Canonical::Test => "test",
            Canonical::Build => "build",
            Canonical::Ci => "ci",
            Canonical::Chore => "chore",
            Canonical::Sec => "sec",
            Canonical::Revert => "revert",
        }
    }

    /// The sequence over the 36 generative tags.
    pub fn sequence(self) -> Vec<Tag> {
        use Concern::*;
        use Mode::*;
        let mk = |c, m| Tag::new(c, m);
        match self {
            Canonical::Feat => vec![
                mk(Structure, Operation),
                mk(Behavior, Type),
                mk(Meaning, Operation),
            ],
            Canonical::Fix => vec![
                mk(Behavior, Operation),
                mk(Correctness, Operation),
                mk(Correctness, Property),
            ],
            Canonical::Refactor => vec![
                mk(Structure, Operation),
                mk(Structure, Invariant),
                mk(Structure, Property),
            ],
            Canonical::Perf => vec![
                mk(Performance, Operation),
                mk(Performance, Invariant),
                mk(Performance, Property),
            ],
            Canonical::Docs => vec![
                mk(Meaning, Type),
                mk(Meaning, Operation),
                mk(Meaning, Property),
            ],
            Canonical::Test => vec![
                mk(Correctness, Type),
                mk(Correctness, Operation),
                mk(Correctness, Property),
            ],
            Canonical::Build => vec![
                mk(Evolution, Operation),
                mk(Performance, Invariant),
                mk(Correctness, Operation),
            ],
            Canonical::Ci => vec![
                mk(Evolution, Operation),
                mk(Correctness, Operation),
                mk(Performance, Property),
            ],
            Canonical::Chore => vec![mk(State, Operation), mk(Evolution, Operation)],
            Canonical::Sec => vec![
                mk(Security, Operation),
                mk(Security, Invariant),
                mk(Security, Property),
            ],
            Canonical::Revert => vec![mk(Evolution, Operation), mk(Evolution, Invariant)],
        }
    }
}

pub const CANONICAL_TAGS: [Canonical; 11] = [
    Canonical::Feat,
    Canonical::Fix,
    Canonical::Refactor,
    Canonical::Perf,
    Canonical::Docs,
    Canonical::Test,
    Canonical::Build,
    Canonical::Ci,
    Canonical::Chore,
    Canonical::Sec,
    Canonical::Revert,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eleven_canonical_tags() {
        assert_eq!(CANONICAL_TAGS.len(), 11);
    }

    #[test]
    fn every_canonical_has_a_sequence() {
        for c in CANONICAL_TAGS {
            let s = c.sequence();
            assert!(!s.is_empty(), "{:?} has empty sequence", c);
        }
    }

    #[test]
    fn names_are_distinct() {
        let mut names: Vec<&str> = CANONICAL_TAGS.iter().map(|c| c.name()).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), 11);
    }
}
