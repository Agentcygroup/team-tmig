#[derive(Clone, Debug)]
pub struct Anchor { pub id: String, pub kind: &'static str, pub label: String }
pub fn all_anchors() -> Vec<Anchor> {
    let mut v = Vec::new();
    for (i, k) in ["lambda", "type", "category", "process", "ontology", "formal", "capability", "agent", "standard"].iter().enumerate() {
        v.push(Anchor { id: format!("{}.{}", k, i), kind: k, label: format!("{} anchor", k) });
    }
    v
}
