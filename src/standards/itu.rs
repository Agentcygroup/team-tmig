use crate::standards::{Domain, Regime, Status};

pub const BODY: &str = "ITU-T";

pub fn regimes() -> Vec<Regime> {
    vec![
        Regime::new(
            "ITU-X-800",
            BODY,
            "Security architecture",
            Domain::Security,
            Status::P,
        ),
        Regime::new(
            "ITU-X-805",
            BODY,
            "Security framework",
            Domain::Security,
            Status::N,
        ),
        Regime::new(
            "ITU-X-509",
            BODY,
            "Public-key infrastructure",
            Domain::Cryptography,
            Status::N,
        ),
        Regime::new(
            "ITU-X-1051",
            BODY,
            "Cybersecurity framework",
            Domain::Security,
            Status::N,
        ),
    ]
}

pub const SELF_REFERENCE: &str =
    "ITU-T recommendations reference each other through the ITU-T A-series.";
