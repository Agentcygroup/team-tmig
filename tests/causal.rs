use tmig::boundary::causal::{classify, CausalBound};

#[test]
fn sufficient_necessary_independent() {
    assert_eq!(classify(1.0, 0.0), CausalBound::Sufficient);
    assert_eq!(classify(1.0, 0.5), CausalBound::Necessary);
    assert_eq!(classify(0.7, 0.2), CausalBound::Contributing);
    assert_eq!(classify(0.3, 0.3), CausalBound::Independent);
}
