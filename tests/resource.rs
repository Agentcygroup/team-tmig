use tmig::boundary::resource::{classify, ResourceBound};

#[test]
fn under_at_over() {
    assert_eq!(classify(5, 10), ResourceBound::Under);
    assert_eq!(classify(10, 10), ResourceBound::At);
    assert_eq!(classify(11, 10), ResourceBound::Over);
}
