use crate::root::types::*;
use crate::root::error::*;

pub trait root_Ops {
    fn id(&self) -> root_Id;
    fn kind(&self) -> root_Kind;
    fn validate(&self) -> root_Result<()>;
}
