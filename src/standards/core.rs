#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Domain {
    Process,
    Security,
    Privacy,
    Risk,
    Governance,
    ItGovernance,
    Ai,
    Software,
    Systems,
    Audit,
    Data,
    Operations,
    ItOps,
    Ops,
    Conformity,
    Cryptography,
    Identity,
    Format,
    Protocol,
    Accessibility,
    Math,
    Vv,
    Architecture,
    Qa,
    Config,
    Test,
    Design,
    Reliability,
    Metrology,
    Physics,
    Accreditation,
    Finance,
    Medical,
    Education,
    Payments,
    Cloud,
    Gov,
    Defense,
    Trade,
    Health,
    Automotive,
    Industrial,
    Safety,
    Aviation,
    Rail,
    Energy,
    SupplyChain,
    Platform,
    Science,
    Biology,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Y,
    P,
    N,
    X,
}

#[derive(Clone, Debug)]
pub struct Regime {
    pub id: String,
    pub body: String,
    pub title: String,
    pub domain: Domain,
    pub status: Status,
}

impl Regime {
    pub fn new(id: &str, body: &str, title: &str, domain: Domain, status: Status) -> Self {
        Self {
            id: id.into(),
            body: body.into(),
            title: title.into(),
            domain,
            status,
        }
    }
}
