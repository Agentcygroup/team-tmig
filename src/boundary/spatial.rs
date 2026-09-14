#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpatialBound {
    Inside,
    Outside,
    OnBoundary,
}

pub fn classify(x: f64, lo: f64, hi: f64) -> SpatialBound {
    if (x - lo).abs() < f64::EPSILON || (x - hi).abs() < f64::EPSILON {
        SpatialBound::OnBoundary
    } else if x > lo && x < hi {
        SpatialBound::Inside
    } else {
        SpatialBound::Outside
    }
}
