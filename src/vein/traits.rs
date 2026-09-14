use crate::vein::types::*;
use crate::vein::error::*;

pub trait vein_Ops {
    fn id(&self) -> vein_Id;
    fn kind(&self) -> vein_Kind;
    fn validate(&self) -> vein_Result<()>;
}
