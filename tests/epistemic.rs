use tmig::boundary::epistemic::{classify, EpistemicBound};

#[test]
fn known_unknown_unknowable() {
    assert_eq!(classify(1, true), EpistemicBound::Known);
    assert_eq!(classify(0, true), EpistemicBound::Unknown);
    assert_eq!(classify(0, false), EpistemicBound::Unknowable);
}
