use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum plane_State {
    Idle,
    Active,
    Degraded,
    Failed,
}

impl plane_State {
    pub fn is_recoverable(self) -> bool {
        matches!(self, Self::Idle | Self::Active | Self::Degraded)
    }
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Failed)
    }
}
