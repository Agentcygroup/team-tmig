use crate::node::types::*;
use crate::node::error::*;

pub trait node_Ops {
    fn id(&self) -> node_Id;
    fn kind(&self) -> node_Kind;
    fn validate(&self) -> node_Result<()>;
}
