#[derive(Clone, Debug)]
pub struct Trace {
    pub item: String,
    pub primary_standard: String,
}
pub fn all_traces() -> Vec<Trace> {
    vec![
        Trace {
            item: "sha256".into(),
            primary_standard: "NIST FIPS 180-4".into(),
        },
        Trace {
            item: "timestamp".into(),
            primary_standard: "ISO 8601-1:2019".into(),
        },
    ]
}
