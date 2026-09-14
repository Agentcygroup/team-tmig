#[derive(Clone, Debug)]
pub struct Calibration {
    pub instrument: String,
    pub against: String,
    pub result: String,
}

pub fn all_calibrations() -> Vec<Calibration> {
    vec![
        Calibration { instrument: "sha2 crate".into(), against: "NIST test vectors".into(), result: "unverified".into() },
        Calibration { instrument: "rustc".into(),   against: "Rust test suite".into(),    result: "unverified".into() },
    ]
}
