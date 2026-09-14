use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct node_Id(pub [u8; 32]);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum node_Kind {
    Default,
    Alternate,
    Reserved,
}
