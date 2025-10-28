
use criterion::{criterion_group, criterion_main, Criterion};

fn add_bench2(c: &mut Criterion) {
    c.bench_function("add", |b| {
        b.iter(|| common::add(2,2));
    });
}


criterion_group!(benches, add_bench2);

criterion_main!(benches);
