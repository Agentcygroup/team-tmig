use crate::swarm::types::*;
use crate::swarm::error::*;

pub trait swarm_Ops {
    fn id(&self) -> swarm_Id;
    fn kind(&self) -> swarm_Kind;
    fn validate(&self) -> swarm_Result<()>;
}
