use tmig::boundary::biological::{classify, BioBound};

#[test]
fn viable_marginal_extinct() {
    assert_eq!(classify(0, 10), BioBound::Extinct);
    assert_eq!(classify(5, 10), BioBound::NonViable);
    assert_eq!(classify(10, 10), BioBound::Marginal);
    assert_eq!(classify(100, 10), BioBound::Viable);
}
