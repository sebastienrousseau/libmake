
extern crate criterion;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate {name};
use self::{name};

fn {name}_benchmark(c: &mut Criterion) {
    c.bench_function("{name}", |b| b.iter(|| {name}::{name}(black_box(42))));
}

criterion_group!(
    benches,
    {name}_benchmark,
);
criterion_main!(benches);