use crate::layer::types::*;
use crate::layer::error::*;

pub trait layer_Ops {
    fn id(&self) -> layer_Id;
    fn kind(&self) -> layer_Kind;
    fn validate(&self) -> layer_Result<()>;
}
