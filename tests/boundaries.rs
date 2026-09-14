#[test]
fn boundary_modules_declared() {
    let n = std::fs::read_dir("src/boundary")
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map(|x| x == "rs").unwrap_or(false))
        .count();
    assert!(n >= 21);
}

#[test]
fn cardinality_classifies() {
    use tmig::boundary::cardinality::{classify, Cardinality};
    assert_eq!(classify(None), Cardinality::Uncountable);
    assert_eq!(classify(Some(0)), Cardinality::Zero);
    assert_eq!(classify(Some(1)), Cardinality::One);
    assert_eq!(classify(Some(2)), Cardinality::Many);
}
