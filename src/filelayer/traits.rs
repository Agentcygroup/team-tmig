use crate::filelayer::types::*;
use crate::filelayer::error::*;

pub trait filelayer_Ops {
    fn id(&self) -> filelayer_Id;
    fn kind(&self) -> filelayer_Kind;
    fn validate(&self) -> filelayer_Result<()>;
}
