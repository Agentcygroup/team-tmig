#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalBound {
    Necessary,
    Possible,
    Impossible,
    Contingent,
}
pub fn classify(p: f32) -> ModalBound {
    if p >= 1.0 {
        ModalBound::Necessary
    } else if p <= 0.0 {
        ModalBound::Impossible
    } else if p > 0.5 {
        ModalBound::Contingent
    } else {
        ModalBound::Possible
    }
}
