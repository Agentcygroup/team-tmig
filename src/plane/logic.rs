use crate::plane::types::*;

pub fn identity(id: &plane_Id) -> plane_Id {
    id.clone()
}

pub fn classify(k: &plane_Kind) -> u8 {
    match k {
        plane_Kind::Default   => 0,
        plane_Kind::Alternate => 1,
        plane_Kind::Reserved  => 2,
    }
}
