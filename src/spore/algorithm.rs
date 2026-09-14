use crate::spore::types::*;

pub fn sort_ids(mut v: Vec<spore_Id>) -> Vec<spore_Id> {
    v.sort_by(|a, b| a.0.cmp(&b.0));
    v
}

pub fn dedupe_ids(v: Vec<spore_Id>) -> Vec<spore_Id> {
    let mut seen = std::collections::BTreeSet::new();
    v.into_iter().filter(|id| seen.insert(id.0)).collect()
}

pub fn find_by_id(v: &[spore_Id], target: &spore_Id) -> Option<usize> {
    v.iter().position(|id| id.0 == target.0)
}
