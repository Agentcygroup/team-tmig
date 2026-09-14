use criterion::{criterion_group, criterion_main, Criterion};
use tmig::boundary::cardinality::classify as card;

fn bench_cardinality(c: &mut Criterion) {
    c.bench_function("cardinality many", |b| b.iter(|| card(Some(42))));
}

criterion_group!(benches, bench_cardinality);
criterion_main!(benches);
