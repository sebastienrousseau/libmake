
extern crate criterion;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate {{LIBNAME}};
use self::{{LIBNAME}};

fn {{LIBNAME}}_benchmark(c: &mut Criterion) {
    c.bench_function("{{LIBNAME}}", |b| b.iter(|| {{LIBNAME}}::{{LIBNAME}}(black_box(42))));
}

criterion_group!(
    benches,
    {{LIBNAME}}_benchmark,
);
criterion_main!(benches);