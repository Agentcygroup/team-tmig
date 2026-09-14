use crate::dunbar::types::*;
use crate::dunbar::error::*;

pub trait dunbar_Ops {
    fn id(&self) -> dunbar_Id;
    fn kind(&self) -> dunbar_Kind;
    fn validate(&self) -> dunbar_Result<()>;
}
