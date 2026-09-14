use tmig::boundary::physical::{classify, PhysicalBound};

#[test]
fn allowed_versus_forbidden() {
    assert_eq!(classify(10.0, 5.0), PhysicalBound::ClassicallyAllowed);
    assert_eq!(classify(1.0, 5.0), PhysicalBound::QuantumForbidden);
}
