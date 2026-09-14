pub const CANONICAL: &[&str] = &[
    "feat", "fix", "refactor", "perf", "docs", "test", "build", "ci", "chore", "sec", "revert",
];
pub fn is_canonical(s: &str) -> bool {
    CANONICAL.contains(&s)
}
