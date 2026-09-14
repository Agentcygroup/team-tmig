#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CryptoBound {
    Secure,
    Marginal,
    Broken,
    Unknown,
}
pub fn classify(bits: u32, quantum_safe: bool) -> CryptoBound {
    if quantum_safe && bits >= 256 {
        CryptoBound::Secure
    } else if bits >= 128 {
        CryptoBound::Marginal
    } else if bits > 0 {
        CryptoBound::Broken
    } else {
        CryptoBound::Unknown
    }
}
