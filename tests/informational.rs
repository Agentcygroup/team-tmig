use tmig::boundary::informational::{classify, InfoBound};

#[test]
fn below_at_above_beyond() {
    assert_eq!(classify(1.0, 10.0), InfoBound::Below);
    assert_eq!(classify(10.0, 10.0), InfoBound::At);
    assert_eq!(classify(15.0, 10.0), InfoBound::Above);
    assert_eq!(classify(30.0, 10.0), InfoBound::Beyond);
}
