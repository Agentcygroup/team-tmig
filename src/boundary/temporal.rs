#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemporalBound {
    Past,
    Present,
    Future,
}

pub fn classify(offset: i32) -> TemporalBound {
    match offset.cmp(&0) {
        std::cmp::Ordering::Less => TemporalBound::Past,
        std::cmp::Ordering::Equal => TemporalBound::Present,
        std::cmp::Ordering::Greater => TemporalBound::Future,
    }
}
