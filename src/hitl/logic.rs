use crate::hitl::types::*;

pub fn identity(id: &hitl_Id) -> hitl_Id {
    id.clone()
}

pub fn classify(k: &hitl_Kind) -> u8 {
    match k {
        hitl_Kind::Default   => 0,
        hitl_Kind::Alternate => 1,
        hitl_Kind::Reserved  => 2,
    }
}
