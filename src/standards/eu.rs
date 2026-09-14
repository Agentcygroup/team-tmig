use crate::standards::{Domain, Regime, Status};

pub const BODY: &str = "European Union";

pub fn regimes() -> Vec<Regime> {
    vec![
        Regime::new(
            "EU-GDPR",
            BODY,
            "Data protection",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "EU-AI-ACT",
            BODY,
            "Artificial Intelligence Act",
            Domain::Ai,
            Status::P,
        ),
        Regime::new(
            "EU-NIS2",
            BODY,
            "Network and Information Security",
            Domain::Security,
            Status::N,
        ),
        Regime::new(
            "EU-DORA",
            BODY,
            "Digital Operational Resilience",
            Domain::Finance,
            Status::N,
        ),
        Regime::new(
            "EU-CRA",
            BODY,
            "Cyber Resilience Act",
            Domain::Security,
            Status::N,
        ),
        Regime::new(
            "EU-MDR",
            BODY,
            "Medical Device Regulation",
            Domain::Medical,
            Status::N,
        ),
        Regime::new(
            "EU-EIDAS",
            BODY,
            "Electronic identification",
            Domain::Identity,
            Status::N,
        ),
        Regime::new(
            "EU-DSA",
            BODY,
            "Digital Services Act",
            Domain::Platform,
            Status::N,
        ),
        Regime::new(
            "EU-DMA",
            BODY,
            "Digital Markets Act",
            Domain::Platform,
            Status::N,
        ),
    ]
}

pub const SELF_REFERENCE: &str =
    "EU regulations reference EU harmonized standards for presumption of conformity.";
