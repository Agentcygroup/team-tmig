use crate::controlplane::types::*;

pub fn identity(id: &controlplane_Id) -> controlplane_Id {
    id.clone()
}

pub fn classify(k: &controlplane_Kind) -> u8 {
    match k {
        controlplane_Kind::Default   => 0,
        controlplane_Kind::Alternate => 1,
        controlplane_Kind::Reserved  => 2,
    }
}
