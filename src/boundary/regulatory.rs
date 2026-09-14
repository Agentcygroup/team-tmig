#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegulatoryBound {
    Required,
    Recommended,
    Discouraged,
    Prohibited,
    Unknown,
}

pub fn classify(phrase: &str) -> RegulatoryBound {
    match phrase.trim().to_lowercase().as_str() {
        "shall" => RegulatoryBound::Required,
        "should" => RegulatoryBound::Recommended,
        "should not" => RegulatoryBound::Discouraged,
        "shall not" => RegulatoryBound::Prohibited,
        _ => RegulatoryBound::Unknown,
    }
}
