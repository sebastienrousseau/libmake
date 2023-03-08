extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};
use {name}::run;

fn {name}_benchmark(c: &mut Criterion) {
    c.bench_function("{name}", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                match run() {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error running {name}: {:?}", e);
                    }
                }
            }
        })
    });
}

criterion_group!(benches, {name}_benchmark);
criterion_main!(benches);
