use crate::standards::{Domain, Regime, Status};

pub const BODY: &str = "W3C";

pub fn regimes() -> Vec<Regime> {
    vec![
        Regime::new(
            "W3C-PROV-DM",
            BODY,
            "Provenance data model",
            Domain::Audit,
            Status::P,
        ),
        Regime::new(
            "W3C-PROV-O",
            BODY,
            "Provenance ontology",
            Domain::Audit,
            Status::N,
        ),
        Regime::new(
            "W3C-DID",
            BODY,
            "Decentralized identifiers",
            Domain::Identity,
            Status::N,
        ),
        Regime::new(
            "W3C-VC",
            BODY,
            "Verifiable credentials",
            Domain::Identity,
            Status::N,
        ),
        Regime::new(
            "W3C-WCAG",
            BODY,
            "Web accessibility",
            Domain::Accessibility,
            Status::N,
        ),
    ]
}

pub const SELF_REFERENCE: &str = "W3C specifications reference each other through the W3C process.";
