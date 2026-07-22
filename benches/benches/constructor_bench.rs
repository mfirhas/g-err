use std::hint::black_box;

use criterion::{Criterion, Throughput};

#[path = "constructor.rs"]
mod constructor;

pub fn bench_all(c: &mut Criterion) {
    builder_bench(c);
    macro_bench(c);
}

pub fn builder_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("builder");

    g.bench_function("default", |b| b.iter(|| black_box(constructor::default())));
    g.bench_function("default_with_metadata", |b| {
        b.iter(|| black_box(constructor::default_with_metadata()))
    });

    g.finish();
}

pub fn macro_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("macro");

    g.bench_function("default_macro", |b| {
        b.iter(|| black_box(constructor::default_macro()))
    });
    g.bench_function("default_with_metadata_macro", |b| {
        b.iter(|| black_box(constructor::default_with_metadata_macro()))
    });

    g.finish();
}
