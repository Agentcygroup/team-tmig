use crate::standards::{Domain, Regime, Status};

pub const BODY: &str = "IEEE";

pub fn regimes() -> Vec<Regime> {
    vec![
        Regime::new(
            "IEEE-754",
            BODY,
            "Floating-point arithmetic",
            Domain::Math,
            Status::Y,
        ),
        Regime::new(
            "IEEE-1012",
            BODY,
            "Verification and validation",
            Domain::Vv,
            Status::P,
        ),
        Regime::new(
            "IEEE-1471",
            BODY,
            "Architecture description",
            Domain::Architecture,
            Status::P,
        ),
        Regime::new(
            "IEEE-730",
            BODY,
            "Software quality assurance",
            Domain::Qa,
            Status::P,
        ),
        Regime::new(
            "IEEE-828",
            BODY,
            "Configuration management",
            Domain::Config,
            Status::P,
        ),
        Regime::new(
            "IEEE-1008",
            BODY,
            "Software unit testing",
            Domain::Test,
            Status::P,
        ),
        Regime::new(
            "IEEE-829",
            BODY,
            "Test documentation",
            Domain::Test,
            Status::N,
        ),
        Regime::new(
            "IEEE-1016",
            BODY,
            "Software design descriptions",
            Domain::Design,
            Status::N,
        ),
        Regime::new(
            "IEEE-1220",
            BODY,
            "Systems engineering",
            Domain::Systems,
            Status::N,
        ),
        Regime::new(
            "IEEE-1633",
            BODY,
            "Software reliability",
            Domain::Reliability,
            Status::N,
        ),
    ]
}

pub const SELF_REFERENCE: &str = "IEEE standards reference each other through the IEEE SA process.";
