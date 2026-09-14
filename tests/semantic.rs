use tmig::boundary::semantic::{classify, SemanticBound};

#[test]
fn well_defined_ambiguous_paradoxical() {
    assert_eq!(classify(1, 0), SemanticBound::WellDefined);
    assert_eq!(classify(2, 0), SemanticBound::Ambiguous);
    assert_eq!(classify(0, 0), SemanticBound::Undefined);
    assert_eq!(classify(1, 1), SemanticBound::Paradoxical);
}
