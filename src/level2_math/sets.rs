use std::collections::BTreeSet;
pub fn is_subset<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> bool {
    a.is_subset(b)
}
pub fn union<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.union(b).cloned().collect()
}
