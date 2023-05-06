extern crate criterion;
extern crate libmake;

use criterion::{criterion_group, criterion_main, Criterion};
use libmake::run;

fn libmake_benchmark(c: &mut Criterion) {
    c.bench_function("libmake", |b| b.iter(run));
}

criterion_group!(benches, libmake_benchmark);
criterion_main!(benches);
