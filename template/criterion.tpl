// SPDX-FileCopyrightText: Copyright © 2023 {name}. All rights reserved.
// SPDX-License-Identifier: {license}

extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};
use {name}::{run, {name}_vec, {name}_map, {name}_join};

fn {name}_vec_benchmark(c: &mut Criterion) {
    c.bench_function("{name}_vec_macro", |b| {
        b.iter(|| {
            {name}_vec![1, 2, 3, 4, 5]
        })
    });
}

fn {name}_map_benchmark(c: &mut Criterion) {
    c.bench_function("{name}_map_macro", |b| {
        b.iter(|| {
            {name}_map!["a" => 1, "b" => 2, "c" => 3, "d" => 4, "e" => 5]
        })
    });
}

fn {name}_join_benchmark(c: &mut Criterion) {
    c.bench_function("{name}_join_macro", |b| {
        b.iter(|| {
            {name}_join!["a", "b", "c", "d", "e"]
        })
    });
}

fn {name}_benchmark(c: &mut Criterion) {
    c.bench_function("{name}", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                run().unwrap();
            }
        })
    });
}

criterion_group!(
    {name}_macros_benchmark,
    {name}_vec_benchmark,
    {name}_map_benchmark,
    {name}_join_benchmark,
    {name}_benchmark
);
criterion_main!({name}_macros_benchmark);