#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeonticBound {
    Obligatory,
    Permitted,
    Forbidden,
    Optional,
}
pub fn classify(p: f32) -> DeonticBound {
    if p >= 1.0 {
        DeonticBound::Obligatory
    } else if p <= 0.0 {
        DeonticBound::Forbidden
    } else if p >= 0.5 {
        DeonticBound::Permitted
    } else {
        DeonticBound::Optional
    }
}
