use tmig::boundary::temporal::{classify, TemporalBound};

#[test]
fn past_present_future() {
    assert_eq!(classify(-1), TemporalBound::Past);
    assert_eq!(classify(0), TemporalBound::Present);
    assert_eq!(classify(1), TemporalBound::Future);
}
