//! # Benchmark: libmake
//!
//! This benchmark tests the performance of the `libmake` crate using the Criterion library.
//! It measures the execution time of the `run` function from the `libmake` crate.
//!
//! ## Purpose
//!
//! The purpose of this benchmark is to assess the execution time of the `run` function
//! from the `libmake` crate under controlled conditions. It helps identify any potential
//! performance bottlenecks and allows for optimization if needed.
//!
//! ## Usage
//!
//! To run this benchmark, ensure that you have the `criterion` and `libmake` crates
//! included as dependencies in your project's `Cargo.toml` file.
//!
//! In your project's code, you can use the `criterion_group` and `criterion_main` macros
//! to define and run benchmarks. In this specific benchmark, the `libmake_benchmark` function
//! is defined to measure the execution time of the `run` function.
//!
//! ```rust
//! extern crate criterion;
//! extern crate libmake;
//!
//! use criterion::{criterion_group, criterion_main, Criterion};
//! use libmake::run;
//!
//! fn libmake_benchmark(c: &mut Criterion) {
//!     c.bench_function("libmake", |b| b.iter(run));
//! }
//!
//! criterion_group!(benches, libmake_benchmark);
//! criterion_main!(benches);
//! ```
//!
//! Running this benchmark will provide performance metrics for the `run` function
//! from the `libmake` crate, helping you evaluate and optimize its performance.

#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main, Criterion};
use libmake::run;

fn libmake_benchmark(c: &mut Criterion) {
    c.bench_function("libmake", |b| b.iter(run));
}

criterion_group!(benches, libmake_benchmark);
criterion_main!(benches);
