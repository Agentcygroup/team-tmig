use crate::vein::types::*;

pub fn identity(id: &vein_Id) -> vein_Id {
    id.clone()
}

pub fn classify(k: &vein_Kind) -> u8 {
    match k {
        vein_Kind::Default   => 0,
        vein_Kind::Alternate => 1,
        vein_Kind::Reserved  => 2,
    }
}
