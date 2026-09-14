use crate::standards::{Domain, Regime, Status};

pub const BODY: &str = "IETF";

pub fn regimes() -> Vec<Regime> {
    vec![
        Regime::new(
            "RFC-2119",
            BODY,
            "Requirement keywords",
            Domain::Process,
            Status::Y,
        ),
        Regime::new(
            "RFC-3339",
            BODY,
            "Date and time on Internet",
            Domain::Format,
            Status::Y,
        ),
        Regime::new(
            "RFC-6234",
            BODY,
            "SHA-2 usage",
            Domain::Cryptography,
            Status::Y,
        ),
        Regime::new("RFC-8259", BODY, "JSON", Domain::Format, Status::Y),
        Regime::new(
            "RFC-7519",
            BODY,
            "JSON Web Token",
            Domain::Identity,
            Status::N,
        ),
        Regime::new("RFC-8446", BODY, "TLS 1.3", Domain::Cryptography, Status::N),
        Regime::new(
            "RFC-9110",
            BODY,
            "HTTP semantics",
            Domain::Protocol,
            Status::N,
        ),
        Regime::new(
            "RFC-3161",
            BODY,
            "Timestamp protocol",
            Domain::Audit,
            Status::N,
        ),
        Regime::new(
            "RFC-6962",
            BODY,
            "Certificate transparency",
            Domain::Audit,
            Status::N,
        ),
    ]
}

pub const SELF_REFERENCE: &str = "IETF RFCs reference each other through the RFC series.";
