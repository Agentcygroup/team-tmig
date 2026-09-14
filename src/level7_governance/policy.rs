#[derive(Clone, Debug)]
pub struct Policy {
    pub id: String,
    pub statement: String,
    pub owner: String,
}
pub fn all_policies() -> Vec<Policy> {
    vec![
        Policy {
            id: "P-1".into(),
            statement: "log every write".into(),
            owner: "operator".into(),
        },
        Policy {
            id: "P-2".into(),
            statement: "two-person approval".into(),
            owner: "architect".into(),
        },
    ]
}
