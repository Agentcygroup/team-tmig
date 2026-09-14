use crate::controlplane::types::*;
use crate::controlplane::error::*;

pub trait controlplane_Ops {
    fn id(&self) -> controlplane_Id;
    fn kind(&self) -> controlplane_Kind;
    fn validate(&self) -> controlplane_Result<()>;
}
