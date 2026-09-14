use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct vein_Id(pub [u8; 32]);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum vein_Kind {
    Default,
    Alternate,
    Reserved,
}
