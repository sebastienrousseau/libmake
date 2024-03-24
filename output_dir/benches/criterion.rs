// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// Copyright © 2024 test_lib. All rights reserved.
// SPDX-License-Identifier: MIT

extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};
use test_lib::{run, test_lib_vec, test_lib_map, test_lib_join};

fn test_lib_vec_benchmark(c: &mut Criterion) {
    c.bench_function("test_lib_vec_macro", |b| {
        b.iter(|| {
            test_lib_vec![1, 2, 3, 4, 5]
        })
    });
}

fn test_lib_map_benchmark(c: &mut Criterion) {
    c.bench_function("test_lib_map_macro", |b| {
        b.iter(|| {
            test_lib_map!["a" => 1, "b" => 2, "c" => 3, "d" => 4, "e" => 5]
        })
    });
}

fn test_lib_join_benchmark(c: &mut Criterion) {
    c.bench_function("test_lib_join_macro", |b| {
        b.iter(|| {
            test_lib_join!["a", "b", "c", "d", "e"]
        })
    });
}

fn test_lib_benchmark(c: &mut Criterion) {
    c.bench_function("test_lib", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                run().unwrap();
            }
        })
    });
}

criterion_group!(
    test_lib_macros_benchmark,
    test_lib_vec_benchmark,
    test_lib_map_benchmark,
    test_lib_join_benchmark,
    test_lib_benchmark
);
criterion_main!(test_lib_macros_benchmark);
