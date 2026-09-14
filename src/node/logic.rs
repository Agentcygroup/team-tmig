use crate::node::types::*;

pub fn identity(id: &node_Id) -> node_Id {
    id.clone()
}

pub fn classify(k: &node_Kind) -> u8 {
    match k {
        node_Kind::Default   => 0,
        node_Kind::Alternate => 1,
        node_Kind::Reserved  => 2,
    }
}
