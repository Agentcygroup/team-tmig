use crate::swarm::types::*;

pub fn identity(id: &swarm_Id) -> swarm_Id {
    id.clone()
}

pub fn classify(k: &swarm_Kind) -> u8 {
    match k {
        swarm_Kind::Default   => 0,
        swarm_Kind::Alternate => 1,
        swarm_Kind::Reserved  => 2,
    }
}
