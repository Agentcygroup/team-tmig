use tmig::boundary::linguistic::{classify, LinguisticBound};

#[test]
fn grammatical_ungrammatical_undefined() {
    assert_eq!(classify(0, true), LinguisticBound::Undefined);
    assert_eq!(classify(5, true), LinguisticBound::Grammatical);
    assert_eq!(classify(5, false), LinguisticBound::Ungrammatical);
}
