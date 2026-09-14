#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatisticalBound {
    Certain,
    Probable,
    Improbable,
    Impossible,
}
pub fn classify(p: f64) -> StatisticalBound {
    if p >= 0.999 {
        StatisticalBound::Certain
    } else if p >= 0.5 {
        StatisticalBound::Probable
    } else if p > 0.001 {
        StatisticalBound::Improbable
    } else {
        StatisticalBound::Impossible
    }
}
