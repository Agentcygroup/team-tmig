use crate::tags::types::*;

pub fn identity(id: &tags_Id) -> tags_Id {
    id.clone()
}

pub fn classify(k: &tags_Kind) -> u8 {
    match k {
        tags_Kind::Default   => 0,
        tags_Kind::Alternate => 1,
        tags_Kind::Reserved  => 2,
    }
}
