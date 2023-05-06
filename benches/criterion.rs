extern crate criterion;
extern crate libmake;

use criterion::{criterion_group, criterion_main, Criterion};
use libmake::run;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("libmake", |b| b.iter(|| run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
