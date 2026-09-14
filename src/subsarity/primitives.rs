#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Primitive { Declaration, Consent, Retention, Disclosure, Provenance, Revocation }
pub fn all() -> Vec<Primitive> {
    vec![Primitive::Declaration, Primitive::Consent, Primitive::Retention, Primitive::Disclosure, Primitive::Provenance, Primitive::Revocation]
}
