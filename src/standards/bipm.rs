use crate::standards::{Domain, Regime, Status};

pub const BODY: &str = "BIPM";

pub fn regimes() -> Vec<Regime> {
    vec![
        Regime::new(
            "SI-Brochure",
            BODY,
            "SI unit definitions",
            Domain::Metrology,
            Status::Y,
        ),
        Regime::new(
            "CODATA-2022",
            BODY,
            "Fundamental constants",
            Domain::Physics,
            Status::Y,
        ),
        Regime::new(
            "BIPM-CCTF",
            BODY,
            "Time and frequency",
            Domain::Metrology,
            Status::N,
        ),
        Regime::new("BIPM-CCL", BODY, "Length", Domain::Metrology, Status::N),
        Regime::new(
            "BIPM-CCT",
            BODY,
            "Thermometry",
            Domain::Metrology,
            Status::N,
        ),
        Regime::new(
            "BIPM-CCQM",
            BODY,
            "Chemistry metrology",
            Domain::Metrology,
            Status::N,
        ),
    ]
}

pub const SELF_REFERENCE: &str = "BIPM realizes the SI through the Consultative Committees.";
