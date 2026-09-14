#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Degradation {
    Full,
    Reduced,
    Minimal,
    ReadOnly,
    Offline,
}

impl Degradation {
    pub fn capabilities(&self) -> u32 {
        match self {
            Self::Full => 100,
            Self::Reduced => 75,
            Self::Minimal => 50,
            Self::ReadOnly => 25,
            Self::Offline => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn monotone() {
        assert!(Degradation::Full.capabilities() > Degradation::Reduced.capabilities());
        assert!(Degradation::Reduced.capabilities() > Degradation::Minimal.capabilities());
    }
}
