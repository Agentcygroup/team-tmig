use crate::standards::{Domain, Regime, Status};

pub const BODY: &str = "US states";

pub fn regimes() -> Vec<Regime> {
    vec![
        Regime::new(
            "CA-CCPA",
            BODY,
            "California Consumer Privacy",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "CA-CPRA",
            BODY,
            "California Privacy Rights",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "VA-CDPSA",
            BODY,
            "Virginia Consumer Data",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "CO-CPA",
            BODY,
            "Colorado Privacy Act",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "CT-DPA",
            BODY,
            "Connecticut Data Privacy",
            Domain::Privacy,
            Status::N,
        ),
        Regime::new(
            "TX-DPSA",
            BODY,
            "Texas Data Privacy",
            Domain::Privacy,
            Status::N,
        ),
    ]
}

pub const SELF_REFERENCE: &str = "US state privacy laws reference each other for reciprocity.";
