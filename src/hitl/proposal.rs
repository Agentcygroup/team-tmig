use crate::hitl::consequence::Consequence;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub description: String,
    pub consequence: Consequence,
}
