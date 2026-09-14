use crate::layer::types::*;

pub fn identity(id: &layer_Id) -> layer_Id {
    id.clone()
}

pub fn classify(k: &layer_Kind) -> u8 {
    match k {
        layer_Kind::Default   => 0,
        layer_Kind::Alternate => 1,
        layer_Kind::Reserved  => 2,
    }
}
