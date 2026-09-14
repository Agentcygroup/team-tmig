use crate::mold::types::*;

pub fn identity(id: &mold_Id) -> mold_Id {
    id.clone()
}

pub fn classify(k: &mold_Kind) -> u8 {
    match k {
        mold_Kind::Default   => 0,
        mold_Kind::Alternate => 1,
        mold_Kind::Reserved  => 2,
    }
}
