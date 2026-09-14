use crate::standards::{Domain, Regime, Status};

pub const BODY: &str = "NIST";

pub fn regimes() -> Vec<Regime> {
    vec![
        Regime::new(
            "NIST-CSF",
            BODY,
            "Cybersecurity Framework",
            Domain::Security,
            Status::P,
        ),
        Regime::new(
            "NIST-800-53",
            BODY,
            "Security and privacy controls",
            Domain::Security,
            Status::P,
        ),
        Regime::new(
            "NIST-800-53A",
            BODY,
            "Control assessment procedures",
            Domain::Audit,
            Status::N,
        ),
        Regime::new(
            "NIST-800-37",
            BODY,
            "Risk management framework",
            Domain::Risk,
            Status::N,
        ),
        Regime::new(
            "NIST-800-92",
            BODY,
            "Log management",
            Domain::Audit,
            Status::P,
        ),
        Regime::new("NIST-800-218", BODY, "SSDF", Domain::Software, Status::P),
        Regime::new(
            "NIST-AI-RMF",
            BODY,
            "AI Risk Management Framework",
            Domain::Ai,
            Status::P,
        ),
        Regime::new(
            "NIST-FIPS-140-3",
            BODY,
            "Cryptographic module validation",
            Domain::Cryptography,
            Status::X,
        ),
        Regime::new(
            "NIST-FIPS-180-4",
            BODY,
            "Secure Hash Standard",
            Domain::Cryptography,
            Status::Y,
        ),
        Regime::new(
            "NIST-FIPS-186-5",
            BODY,
            "Digital Signature Standard",
            Domain::Cryptography,
            Status::Y,
        ),
        Regime::new(
            "NIST-FIPS-197",
            BODY,
            "AES",
            Domain::Cryptography,
            Status::N,
        ),
        Regime::new(
            "NIST-FIPS-202",
            BODY,
            "SHA-3",
            Domain::Cryptography,
            Status::N,
        ),
        Regime::new(
            "NIST-FIPS-203",
            BODY,
            "ML-KEM",
            Domain::Cryptography,
            Status::N,
        ),
        Regime::new(
            "NIST-FIPS-204",
            BODY,
            "ML-DSA",
            Domain::Cryptography,
            Status::N,
        ),
        Regime::new(
            "NIST-FIPS-205",
            BODY,
            "SLH-DSA",
            Domain::Cryptography,
            Status::N,
        ),
    ]
}

pub const SELF_REFERENCE: &str = "NIST validates NIST FIPS modules through NIST FIPS 140-3.";
