#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BioBound {
    Extinct,
    NonViable,
    Marginal,
    Viable,
}

pub fn classify(population: u32, capacity: u32) -> BioBound {
    if population == 0 {
        BioBound::Extinct
    } else if (population as u64) * 2 <= capacity as u64 {
        BioBound::NonViable
    } else if population <= capacity {
        BioBound::Marginal
    } else {
        BioBound::Viable
    }
}
