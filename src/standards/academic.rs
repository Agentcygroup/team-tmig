use crate::standards::{Domain, Regime, Status};

pub const BODY: &str = "Academic";

pub fn regimes() -> Vec<Regime> {
    vec![
        Regime::new(
            "ACM-Artifact",
            BODY,
            "ACM Artifact Review",
            Domain::Science,
            Status::N,
        ),
        Regime::new(
            "IEEE-Reproducibility",
            BODY,
            "Reproducibility Badge",
            Domain::Science,
            Status::N,
        ),
        Regime::new(
            "FORCE11-FAIR",
            BODY,
            "FAIR Data Principles",
            Domain::Data,
            Status::N,
        ),
        Regime::new(
            "OSF-Prereg",
            BODY,
            "Preregistration",
            Domain::Science,
            Status::N,
        ),
        Regime::new(
            "CONSORT",
            BODY,
            "Clinical trial reporting",
            Domain::Medical,
            Status::N,
        ),
        Regime::new(
            "PRISMA",
            BODY,
            "Systematic review",
            Domain::Medical,
            Status::N,
        ),
        Regime::new(
            "STROBE",
            BODY,
            "Observational studies",
            Domain::Medical,
            Status::N,
        ),
        Regime::new(
            "ARRIVE",
            BODY,
            "Animal research",
            Domain::Biology,
            Status::N,
        ),
    ]
}

pub const SELF_REFERENCE: &str = "Academic regimes reference the Declaration of Helsinki.";
