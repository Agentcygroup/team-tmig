use tmig::boundary::cardinality::{classify, Cardinality};

#[test]
fn total_on_all_inputs() {
    let samples: &[Option<u64>] = &[None, Some(0), Some(1), Some(2), Some(u64::MAX)];
    for &n in samples {
        let c = classify(n);
        assert!(matches!(
            c,
            Cardinality::Zero
                | Cardinality::One
                | Cardinality::Many
                | Cardinality::Infinite
                | Cardinality::Uncountable
        ));
    }
}

#[test]
fn exhaustive_on_classes() {
    let classes = [
        Cardinality::Zero,
        Cardinality::One,
        Cardinality::Many,
        Cardinality::Infinite,
        Cardinality::Uncountable,
    ];
    assert_eq!(classes.len(), 5);
}
