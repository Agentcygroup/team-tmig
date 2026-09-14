#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EconomicBound {
    Profitable,
    BreakEven,
    Unprofitable,
    Undefined,
}
pub fn classify(revenue: f64, cost: f64) -> EconomicBound {
    if cost <= 0.0 {
        EconomicBound::Undefined
    } else if revenue > cost {
        EconomicBound::Profitable
    } else if (revenue - cost).abs() < 1e-9 {
        EconomicBound::BreakEven
    } else {
        EconomicBound::Unprofitable
    }
}
