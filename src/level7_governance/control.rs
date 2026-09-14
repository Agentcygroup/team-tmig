#[derive(Clone, Debug)]
pub struct Control {
    pub id: String,
    pub domain: String,
    pub implemented: bool,
    pub location: String,
}

pub fn all_controls() -> Vec<Control> {
    vec![
        Control { id: "A.5.1".into(), domain: "policy".into(),  implemented: true,  location: "src/level7_governance/policy.rs".into() },
        Control { id: "A.8.13".into(),domain: "backup".into(),  implemented: false, location: "—".into() },
        Control { id: "A.8.15".into(),domain: "logging".into(), implemented: false, location: "—".into() },
        Control { id: "A.5.15".into(),domain: "access".into(),  implemented: true,  location: "src/hitl/approval.rs".into() },
        Control { id: "A.8.24".into(),domain: "crypto".into(),  implemented: true,  location: "src/level3_algorithms/hash.rs".into() },
    ]
}
