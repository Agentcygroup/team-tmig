use tmig::boundary::economic::{classify, EconomicBound};

#[test]
fn profitable_breakeven_unprofitable() {
    assert_eq!(classify(10.0, 5.0), EconomicBound::Profitable);
    assert_eq!(classify(5.0, 5.0), EconomicBound::BreakEven);
    assert_eq!(classify(1.0, 5.0), EconomicBound::Unprofitable);
}
