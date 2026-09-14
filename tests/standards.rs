use tmig::standards::self_reference::{
    all_self_references, count_by_status, self_referential_closure, total_regimes,
};

#[test]
fn every_body_has_a_self_reference() {
    for (body, sr) in all_self_references() {
        assert!(!sr.is_empty(), "{} has no self-reference", body);
    }
}

#[test]
fn closure_is_total() {
    assert_eq!(self_referential_closure(), 14);
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
