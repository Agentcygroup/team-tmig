#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Concept,
    Development,
    Production,
    Utilization,
    Support,
    Retirement,
}
pub fn current() -> Phase {
    Phase::Development
}
