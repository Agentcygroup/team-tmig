use crate::hitl::types::*;
use crate::hitl::error::*;

pub trait hitl_Ops {
    fn id(&self) -> hitl_Id;
    fn kind(&self) -> hitl_Kind;
    fn validate(&self) -> hitl_Result<()>;
}
