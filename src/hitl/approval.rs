use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Approval {
    pub proposal_id: String,
    pub approver: String,
    pub signature: String,
}
