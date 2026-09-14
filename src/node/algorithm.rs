use crate::node::types::*;

pub fn sort_ids(mut v: Vec<node_Id>) -> Vec<node_Id> {
    v.sort_by(|a, b| a.0.cmp(&b.0));
    v
}

pub fn dedupe_ids(v: Vec<node_Id>) -> Vec<node_Id> {
    let mut seen = std::collections::BTreeSet::new();
    v.into_iter().filter(|id| seen.insert(id.0)).collect()
}

pub fn find_by_id(v: &[node_Id], target: &node_Id) -> Option<usize> {
    v.iter().position(|id| id.0 == target.0)
}
