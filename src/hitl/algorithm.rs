use crate::hitl::types::*;

pub fn sort_ids(mut v: Vec<hitl_Id>) -> Vec<hitl_Id> {
    v.sort_by(|a, b| a.0.cmp(&b.0));
    v
}

pub fn dedupe_ids(v: Vec<hitl_Id>) -> Vec<hitl_Id> {
    let mut seen = std::collections::BTreeSet::new();
    v.into_iter().filter(|id| seen.insert(id.0)).collect()
}

pub fn find_by_id(v: &[hitl_Id], target: &hitl_Id) -> Option<usize> {
    v.iter().position(|id| id.0 == target.0)
}
