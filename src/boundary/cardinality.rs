#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardinality {
    Zero,
    One,
    Many,
    Infinite,
    Uncountable,
}

pub fn classify(n: Option<u64>) -> Cardinality {
    match n {
        None => Cardinality::Uncountable,
        Some(0) => Cardinality::Zero,
        Some(1) => Cardinality::One,
        _ => Cardinality::Many,
    }
}
