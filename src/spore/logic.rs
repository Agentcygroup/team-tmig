use crate::spore::types::*;

pub fn identity(id: &spore_Id) -> spore_Id {
    id.clone()
}

pub fn classify(k: &spore_Kind) -> u8 {
    match k {
        spore_Kind::Default   => 0,
        spore_Kind::Alternate => 1,
        spore_Kind::Reserved  => 2,
    }
}
