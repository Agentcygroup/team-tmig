use crate::spore::types::*;
use crate::spore::error::*;

pub trait spore_Ops {
    fn id(&self) -> spore_Id;
    fn kind(&self) -> spore_Kind;
    fn validate(&self) -> spore_Result<()>;
}
