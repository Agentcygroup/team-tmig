#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceBound {
    Under,
    At,
    Over,
}

pub fn classify(used: u64, limit: u64) -> ResourceBound {
    match used.cmp(&limit) {
        std::cmp::Ordering::Less => ResourceBound::Under,
        std::cmp::Ordering::Equal => ResourceBound::At,
        std::cmp::Ordering::Greater => ResourceBound::Over,
    }
}
