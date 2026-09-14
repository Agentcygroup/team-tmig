use crate::dunbar::types::*;

pub fn identity(id: &dunbar_Id) -> dunbar_Id {
    id.clone()
}

pub fn classify(k: &dunbar_Kind) -> u8 {
    match k {
        dunbar_Kind::Default   => 0,
        dunbar_Kind::Alternate => 1,
        dunbar_Kind::Reserved  => 2,
    }
}
