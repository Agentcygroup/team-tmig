#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MethodBound {
    Deductive,
    Inductive,
    Abductive,
    Invalid,
}
pub fn classify(premises: u32, conclusion_supported: bool) -> MethodBound {
    if premises == 0 {
        MethodBound::Invalid
    } else if conclusion_supported {
        MethodBound::Deductive
    } else {
        MethodBound::Inductive
    }
}
