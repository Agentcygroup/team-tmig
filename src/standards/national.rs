use crate::standards::{Domain, Regime, Status};

pub const BODY: &str = "National regulators";

pub fn regimes() -> Vec<Regime> {
    vec![
        Regime::new(
            "UK-GDPR",
            BODY,
            "UK data protection",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "CA-PIPEDA",
            BODY,
            "Canadian privacy",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "BR-LGPD",
            BODY,
            "Brazilian privacy",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "JP-APPI",
            BODY,
            "Japanese privacy",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "AU-Privacy-Act",
            BODY,
            "Australian privacy",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "IN-DPDP",
            BODY,
            "Indian privacy",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "CN-PIPL",
            BODY,
            "Chinese privacy",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "KR-PIPA",
            BODY,
            "Korean privacy",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "SG-PDPA",
            BODY,
            "Singapore privacy",
            Domain::Privacy,
            Status::N,
        ),
    ]
}

pub const SELF_REFERENCE: &str = "National privacy laws reference the OECD Privacy Guidelines.";
