#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhysicalBound {
    ClassicallyAllowed,
    QuantumForbidden,
    ClassicallyForbidden,
}
pub fn classify(energy: f64, barrier: f64) -> PhysicalBound {
    if energy >= barrier {
        PhysicalBound::ClassicallyAllowed
    } else {
        PhysicalBound::QuantumForbidden
    }
}
