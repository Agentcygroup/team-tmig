use tmig::boundary::regulatory::{classify, RegulatoryBound};

#[test]
fn shall_should_prohibited() {
    assert_eq!(classify("shall"), RegulatoryBound::Required);
    assert_eq!(classify("should"), RegulatoryBound::Recommended);
    assert_eq!(classify("should not"), RegulatoryBound::Discouraged);
    assert_eq!(classify("shall not"), RegulatoryBound::Prohibited);
}
