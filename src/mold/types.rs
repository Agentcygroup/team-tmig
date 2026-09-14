use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct mold_Id(pub [u8; 32]);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum mold_Kind {
    Default,
    Alternate,
    Reserved,
}
