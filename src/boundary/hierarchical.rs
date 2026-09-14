#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HierarchicalBound {
    Root,
    Intermediate,
    Leaf,
    Orphan,
}
pub fn classify(depth: u32, children: u32) -> HierarchicalBound {
    if depth == 0 && children == 0 {
        HierarchicalBound::Orphan
    } else if depth == 0 {
        HierarchicalBound::Root
    } else if children == 0 {
        HierarchicalBound::Leaf
    } else {
        HierarchicalBound::Intermediate
    }
}
