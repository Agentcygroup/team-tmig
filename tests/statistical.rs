use tmig::boundary::statistical::{classify, StatisticalBound};

#[test]
fn certain_probable_impossible() {
    assert_eq!(classify(0.999), StatisticalBound::Certain);
    assert_eq!(classify(0.7), StatisticalBound::Probable);
    assert_eq!(classify(0.01), StatisticalBound::Improbable);
    assert_eq!(classify(0.0), StatisticalBound::Impossible);
}
