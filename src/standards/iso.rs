use crate::standards::{Domain, Regime, Status};

pub const BODY: &str = "ISO/IEC";

pub fn regimes() -> Vec<Regime> {
    vec![
        Regime::new(
            "ISO-9001",
            BODY,
            "Quality management",
            Domain::Process,
            Status::N,
        ),
        Regime::new(
            "ISO-27001",
            BODY,
            "Information security management",
            Domain::Security,
            Status::P,
        ),
        Regime::new(
            "ISO-27002",
            BODY,
            "Information security controls",
            Domain::Security,
            Status::P,
        ),
        Regime::new(
            "ISO-27701",
            BODY,
            "Privacy information management",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "ISO-31000",
            BODY,
            "Risk management",
            Domain::Risk,
            Status::N,
        ),
        Regime::new(
            "ISO-37000",
            BODY,
            "Governance of organizations",
            Domain::Governance,
            Status::P,
        ),
        Regime::new(
            "ISO-38500",
            BODY,
            "Governance of IT",
            Domain::ItGovernance,
            Status::P,
        ),
        Regime::new(
            "ISO-42001",
            BODY,
            "AI management systems",
            Domain::Ai,
            Status::P,
        ),
        Regime::new(
            "ISO-12207",
            BODY,
            "Software life cycle",
            Domain::Software,
            Status::P,
        ),
        Regime::new(
            "ISO-15288",
            BODY,
            "Systems life cycle",
            Domain::Systems,
            Status::P,
        ),
        Regime::new(
            "ISO-15408",
            BODY,
            "Common Criteria",
            Domain::Security,
            Status::N,
        ),
        Regime::new(
            "ISO-19011",
            BODY,
            "Auditing management systems",
            Domain::Audit,
            Status::N,
        ),
        Regime::new(
            "ISO-25010",
            BODY,
            "Systems and software quality",
            Domain::Software,
            Status::P,
        ),
        Regime::new(
            "ISO-25012",
            BODY,
            "Data quality model",
            Domain::Data,
            Status::N,
        ),
        Regime::new(
            "ISO-29119",
            BODY,
            "Software testing",
            Domain::Software,
            Status::P,
        ),
        Regime::new(
            "ISO-22301",
            BODY,
            "Business continuity",
            Domain::Operations,
            Status::N,
        ),
        Regime::new(
            "ISO-20000",
            BODY,
            "IT service management",
            Domain::ItOps,
            Status::N,
        ),
        Regime::new(
            "ISO-55001",
            BODY,
            "Asset management",
            Domain::Ops,
            Status::N,
        ),
        Regime::new(
            "ISO-IEC-17021",
            BODY,
            "Certification body requirements",
            Domain::Conformity,
            Status::X,
        ),
        Regime::new(
            "ISO-IEC-17025",
            BODY,
            "Testing and calibration labs",
            Domain::Conformity,
            Status::X,
        ),
        Regime::new(
            "ISO-IEC-17065",
            BODY,
            "Product certification bodies",
            Domain::Conformity,
            Status::X,
        ),
    ]
}

pub const SELF_REFERENCE: &str = "ISO/IEC evaluates ISO/IEC regimes through ISO/IEC 17021.";
