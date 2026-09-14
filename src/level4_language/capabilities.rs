#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Capability {
    NoStd,
    NoUnwind,
    DeterministicDrop,
    NoFloatingPointInIso,
    BoundedAllocation,
}

pub fn all_capabilities() -> &'static [Capability] {
    &[
        Capability::NoUnwind,
        Capability::DeterministicDrop,
        Capability::BoundedAllocation,
    ]
}
