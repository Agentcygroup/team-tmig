use crate::dunbar::types::*;

pub fn sort_ids(mut v: Vec<dunbar_Id>) -> Vec<dunbar_Id> {
    v.sort_by(|a, b| a.0.cmp(&b.0));
    v
}

pub fn dedupe_ids(v: Vec<dunbar_Id>) -> Vec<dunbar_Id> {
    let mut seen = std::collections::BTreeSet::new();
    v.into_iter().filter(|id| seen.insert(id.0)).collect()
}

pub fn find_by_id(v: &[dunbar_Id], target: &dunbar_Id) -> Option<usize> {
    v.iter().position(|id| id.0 == target.0)
}
