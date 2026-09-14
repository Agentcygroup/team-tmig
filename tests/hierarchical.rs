use tmig::boundary::hierarchical::{classify, HierarchicalBound};

#[test]
fn root_leaf_orphan_intermediate() {
    assert_eq!(classify(0, 1), HierarchicalBound::Root);
    assert_eq!(classify(1, 0), HierarchicalBound::Leaf);
    assert_eq!(classify(0, 0), HierarchicalBound::Orphan);
    assert_eq!(classify(1, 1), HierarchicalBound::Intermediate);
}
