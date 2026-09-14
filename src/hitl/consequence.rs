use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Consequence {
    None,
    Low,
    Moderate,
    High,
    Legal,
    Irreversible,
}

impl Consequence {
    pub fn hitl_level(self) -> u8 {
        match self {
            Self::None => 5,
            Self::Low => 4,
            Self::Moderate => 3,
            Self::High => 2,
            Self::Legal => 1,
            Self::Irreversible => 0,
        }
    }
}
