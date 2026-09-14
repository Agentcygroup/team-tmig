use crate::mold::types::*;
use crate::mold::error::*;

pub trait mold_Ops {
    fn id(&self) -> mold_Id;
    fn kind(&self) -> mold_Kind;
    fn validate(&self) -> mold_Result<()>;
}
