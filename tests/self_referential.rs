use tmig::boundary::self_referential::{classify, SelfRefBound};

#[test]
fn base_fixed_divergent() {
    assert_eq!(classify(0, 3), SelfRefBound::Base);
    assert_eq!(classify(1, 3), SelfRefBound::FixedPoint);
    assert_eq!(classify(3, 3), SelfRefBound::Divergent);
}
