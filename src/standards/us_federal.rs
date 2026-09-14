use crate::standards::{Domain, Regime, Status};

pub const BODY: &str = "United States federal";

pub fn regimes() -> Vec<Regime> {
    vec![
        Regime::new("US-SOX", BODY, "Sarbanes-Oxley", Domain::Finance, Status::N),
        Regime::new(
            "US-HIPAA",
            BODY,
            "Health Insurance Portability",
            Domain::Health,
            Status::N,
        ),
        Regime::new(
            "US-FERPA",
            BODY,
            "Education records",
            Domain::Education,
            Status::N,
        ),
        Regime::new(
            "US-COPPA",
            BODY,
            "Children privacy",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "US-PCI-DSS",
            BODY,
            "Payment Card Industry",
            Domain::Payments,
            Status::N,
        ),
        Regime::new(
            "US-FedRAMP",
            BODY,
            "Federal cloud authorization",
            Domain::Cloud,
            Status::N,
        ),
        Regime::new(
            "US-NIST-800-171",
            BODY,
            "Controlled Unclassified Info",
            Domain::Gov,
            Status::P,
        ),
        Regime::new(
            "US-CMMC",
            BODY,
            "Cybersecurity Maturity Model",
            Domain::Defense,
            Status::N,
        ),
        Regime::new(
            "US-ITAR",
            BODY,
            "International Traffic in Arms",
            Domain::Defense,
            Status::N,
        ),
        Regime::new(
            "US-EAR",
            BODY,
            "Export Administration",
            Domain::Trade,
            Status::N,
        ),
    ]
}

pub const SELF_REFERENCE: &str =
    "US federal regulations reference NIST publications for technical guidance.";
