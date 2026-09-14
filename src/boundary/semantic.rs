#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticBound {
    WellDefined,
    Ambiguous,
    Undefined,
    Paradoxical,
}

pub fn classify(a: i32, b: i32) -> SemanticBound {
    match (a, b) {
        (1, 0) => SemanticBound::WellDefined,
        (2, 0) => SemanticBound::Ambiguous,
        (0, 0) => SemanticBound::Undefined,
        (1, 1) => SemanticBound::Paradoxical,
        _ => SemanticBound::Undefined,
    }
}
