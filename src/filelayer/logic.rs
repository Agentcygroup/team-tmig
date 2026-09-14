use crate::filelayer::types::*;

pub fn identity(id: &filelayer_Id) -> filelayer_Id {
    id.clone()
}

pub fn classify(k: &filelayer_Kind) -> u8 {
    match k {
        filelayer_Kind::Default   => 0,
        filelayer_Kind::Alternate => 1,
        filelayer_Kind::Reserved  => 2,
    }
}
