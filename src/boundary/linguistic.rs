#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinguisticBound {
    Grammatical,
    Ungrammatical,
    Ambiguous,
    Undefined,
}
pub fn classify(tokens: u32, parse_success: bool) -> LinguisticBound {
    if tokens == 0 {
        LinguisticBound::Undefined
    } else if parse_success {
        LinguisticBound::Grammatical
    } else {
        LinguisticBound::Ungrammatical
    }
}
