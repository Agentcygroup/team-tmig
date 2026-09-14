use crate::standards::{Domain, Regime, Status};

pub const BODY: &str = "ILAC / IAF / CASCO";

pub fn regimes() -> Vec<Regime> {
    vec![
        Regime::new(
            "ILAC-G24",
            BODY,
            "Scope of accreditation",
            Domain::Accreditation,
            Status::P,
        ),
        Regime::new(
            "ILAC-P10",
            BODY,
            "Assessment",
            Domain::Accreditation,
            Status::N,
        ),
        Regime::new(
            "ILAC-P14",
            BODY,
            "Traceability",
            Domain::Accreditation,
            Status::N,
        ),
        Regime::new(
            "IAF-MD1",
            BODY,
            "Certification documents",
            Domain::Conformity,
            Status::N,
        ),
        Regime::new(
            "IAF-MD5",
            BODY,
            "Auditor competence",
            Domain::Conformity,
            Status::N,
        ),
        Regime::new(
            "IAF-MD11",
            BODY,
            "Accreditation assessment",
            Domain::Conformity,
            Status::N,
        ),
        Regime::new(
            "CASCO-17021",
            BODY,
            "Certification body requirements",
            Domain::Conformity,
            Status::N,
        ),
    ]
}

pub const SELF_REFERENCE: &str =
    "ILAC accredits accreditation bodies that accredit certification bodies.";
