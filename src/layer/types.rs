use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct layer_Id(pub [u8; 32]);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum layer_Kind {
    Default,
    Alternate,
    Reserved,
}
