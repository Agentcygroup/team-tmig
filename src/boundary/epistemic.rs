#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EpistemicBound {
    Known,
    Unknown,
    Unknowable,
    Contested,
}
pub fn classify(evidence_count: u64, is_decidable: bool) -> EpistemicBound {
    if !is_decidable {
        return EpistemicBound::Unknowable;
    }
    if evidence_count == 0 {
        return EpistemicBound::Unknown;
    }
    EpistemicBound::Known
}
