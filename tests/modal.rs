use tmig::boundary::modal::{classify, ModalBound};

#[test]
fn necessary_possible_impossible() {
    assert_eq!(classify(1.0), ModalBound::Necessary);
    assert_eq!(classify(0.0), ModalBound::Impossible);
    assert_eq!(classify(0.4), ModalBound::Possible);
    assert_eq!(classify(0.7), ModalBound::Contingent);
}
