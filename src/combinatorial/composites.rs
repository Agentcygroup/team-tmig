#[derive(Clone, Debug)]
pub struct Composite {
    pub domain: String,
    pub unit: String,
}
pub fn all_composites() -> Vec<Composite> {
    vec![Composite {
        domain: "correct".into(),
        unit: "test".into(),
    }]
}
