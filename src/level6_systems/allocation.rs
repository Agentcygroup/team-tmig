#[derive(Clone, Debug)]
pub struct Allocation {
    pub requirement_id: String,
    pub module: String,
}

pub fn all_allocations() -> Vec<Allocation> {
    vec![
        Allocation { requirement_id: "R-1".into(), module: "Cargo.toml".into() },
        Allocation { requirement_id: "R-2".into(), module: "src/bin".into() },
        Allocation { requirement_id: "R-4".into(), module: "src".into() },
    ]
}
