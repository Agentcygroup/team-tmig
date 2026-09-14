#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AreaCode(pub u16);
impl AreaCode {
    pub fn new(code: u16) -> Option<Self> {
        if (200..=999).contains(&code) {
            Some(Self(code))
        } else {
            None
        }
    }
}
