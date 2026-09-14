#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InfoBound {
    Below,
    At,
    Above,
    Beyond,
}
pub fn classify(bits: f64, capacity: f64) -> InfoBound {
    if bits < capacity {
        InfoBound::Below
    } else if (bits - capacity).abs() < 1e-9 {
        InfoBound::At
    } else if bits < 2.0 * capacity {
        InfoBound::Above
    } else {
        InfoBound::Beyond
    }
}
