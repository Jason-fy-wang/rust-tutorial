
use criterion::{criterion_group,criterion_main,Criterion};


fn multipy_bench(c: &mut Criterion) {
    c.bench_function("multipy", |b| {b.iter(|| common::service::multipy(10, 20));});
}


criterion_group!(benches, multipy_bench);
criterion_main!(benches);




