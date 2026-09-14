#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelfRefBound {
    Base,
    FixedPoint,
    Divergent,
}

pub fn classify(depth: u32, cap: u32) -> SelfRefBound {
    if depth == 0 {
        SelfRefBound::Base
    } else if depth < cap {
        SelfRefBound::FixedPoint
    } else {
        SelfRefBound::Divergent
    }
}
