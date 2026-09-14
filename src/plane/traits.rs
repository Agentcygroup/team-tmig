use crate::plane::types::*;
use crate::plane::error::*;

pub trait plane_Ops {
    fn id(&self) -> plane_Id;
    fn kind(&self) -> plane_Kind;
    fn validate(&self) -> plane_Result<()>;
}
