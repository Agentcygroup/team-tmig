use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct filelayer_Id(pub [u8; 32]);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum filelayer_Kind {
    Default,
    Alternate,
    Reserved,
}
