#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CausalBound {
    Sufficient,
    Necessary,
    Contributing,
    Independent,
}
pub fn classify(p_a_given_b: f32, p_a_given_not_b: f32) -> CausalBound {
    if p_a_given_b >= 1.0 && p_a_given_not_b <= 0.0 {
        CausalBound::Sufficient
    } else if p_a_given_b >= 1.0 {
        CausalBound::Necessary
    } else if p_a_given_b > p_a_given_not_b {
        CausalBound::Contributing
    } else {
        CausalBound::Independent
    }
}
