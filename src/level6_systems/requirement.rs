#[derive(Clone, Debug)]
pub struct Requirement {
    pub id: String,
    pub statement: String,
    pub mandatory: bool,
}
pub fn all() -> Vec<Requirement> {
    vec![
        Requirement {
            id: "R-1".into(),
            statement: "build succeeds".into(),
            mandatory: true,
        },
        Requirement {
            id: "R-2".into(),
            statement: "tests pass".into(),
            mandatory: true,
        },
    ]
}
