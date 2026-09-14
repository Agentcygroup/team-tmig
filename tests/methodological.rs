use tmig::boundary::methodological::{classify, MethodBound};

#[test]
fn deductive_inductive_invalid() {
    assert_eq!(classify(0, true), MethodBound::Invalid);
    assert_eq!(classify(3, true), MethodBound::Deductive);
    assert_eq!(classify(3, false), MethodBound::Inductive);
}
