#[derive(Clone, Debug)]
pub struct Accreditation {
    pub body: String,
    pub scope: String,
    pub status: String,
}

pub fn all_accreditations() -> Vec<Accreditation> {
    vec![
        Accreditation { body: "ILAC".into(), scope: "cryptographic module testing".into(), status: "not accredited".into() },
        Accreditation { body: "IAF".into(),  scope: "software process certification".into(), status: "not accredited".into() },
        Accreditation { body: "CASCO".into(),scope: "conformity assessment".into(),        status: "not accredited".into() },
    ]
}
