use crate::root::types::*;

pub fn identity(id: &root_Id) -> root_Id {
    id.clone()
}

pub fn classify(k: &root_Kind) -> u8 {
    match k {
        root_Kind::Default   => 0,
        root_Kind::Alternate => 1,
        root_Kind::Reserved  => 2,
    }
}
