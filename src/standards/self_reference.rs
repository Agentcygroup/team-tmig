use crate::standards::{
    academic, bipm, eu, ieee, ietf, ilac, industry, iso, itu, national, nist, us_federal, us_state,
    w3c,
};

pub fn all_self_references() -> Vec<(&'static str, &'static str)> {
    vec![
        ("ISO/IEC", iso::SELF_REFERENCE),
        ("NIST", nist::SELF_REFERENCE),
        ("IETF", ietf::SELF_REFERENCE),
        ("W3C", w3c::SELF_REFERENCE),
        ("IEEE", ieee::SELF_REFERENCE),
        ("ITU-T", itu::SELF_REFERENCE),
        ("BIPM", bipm::SELF_REFERENCE),
        ("ILAC/IAF", ilac::SELF_REFERENCE),
        ("EU", eu::SELF_REFERENCE),
        ("US federal", us_federal::SELF_REFERENCE),
        ("US states", us_state::SELF_REFERENCE),
        ("National", national::SELF_REFERENCE),
        ("Industry", industry::SELF_REFERENCE),
        ("Academic", academic::SELF_REFERENCE),
    ]
}

pub fn total_regimes() -> usize {
    iso::regimes().len()
        + nist::regimes().len()
        + ietf::regimes().len()
        + w3c::regimes().len()
        + ieee::regimes().len()
        + itu::regimes().len()
        + bipm::regimes().len()
        + ilac::regimes().len()
        + eu::regimes().len()
        + us_federal::regimes().len()
        + us_state::regimes().len()
        + national::regimes().len()
        + industry::regimes().len()
        + academic::regimes().len()
}

pub fn count_by_status() -> (usize, usize, usize, usize) {
    let mut y = 0;
    let mut p = 0;
    let mut n = 0;
    let mut x = 0;
    let all = iso::regimes()
        .into_iter()
        .chain(nist::regimes())
        .chain(ietf::regimes())
        .chain(w3c::regimes())
        .chain(ieee::regimes())
        .chain(itu::regimes())
        .chain(bipm::regimes())
        .chain(ilac::regimes())
        .chain(eu::regimes())
        .chain(us_federal::regimes())
        .chain(us_state::regimes())
        .chain(national::regimes())
        .chain(industry::regimes())
        .chain(academic::regimes());
    for r in all {
        match r.status {
            crate::standards::core::Status::Y => y += 1,
            crate::standards::core::Status::P => p += 1,
            crate::standards::core::Status::N => n += 1,
            crate::standards::core::Status::X => x += 1,
        }
    }
    (y, p, n, x)
}

pub fn self_referential_closure() -> usize {
    all_self_references()
        .iter()
        .filter(|(_, s)| !s.is_empty())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_body_has_a_self_reference() {
        for (body, sr) in all_self_references() {
            assert!(!sr.is_empty(), "{} has no self-reference", body);
        }
    }

    #[test]
    fn closure_is_total() {
        let n = self_referential_closure();
        assert_eq!(n, 14);
    }

    #[test]
    fn counts_are_consistent() {
        let total = total_regimes();
        let (y, p, n, x) = count_by_status();
        assert_eq!(y + p + n + x, total);
    }

    #[test]
    fn total_is_at_least_145() {
        assert!(total_regimes() >= 145);
    }
}
