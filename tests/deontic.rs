use tmig::boundary::deontic::{classify, DeonticBound};

#[test]
fn obligatory_optional_forbidden() {
    assert_eq!(classify(1.0), DeonticBound::Obligatory);
    assert_eq!(classify(0.0), DeonticBound::Forbidden);
    assert_eq!(classify(0.7), DeonticBound::Permitted);
    assert_eq!(classify(0.2), DeonticBound::Optional);
}
