use std::hint::black_box;

use criterion::{BenchmarkId, Criterion};

#[path = "constructor.rs"]
mod constructor;

pub fn bench_all(c: &mut Criterion) {
    constructor_default_bench(c);
}

pub fn constructor_default_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("constructor");

    g.bench_function(BenchmarkId::new("default", "builder"), |b| {
        b.iter(|| black_box(constructor::default()))
    });

    g.bench_function(BenchmarkId::new("default", "macro"), |b| {
        b.iter(|| black_box(constructor::default_macro()))
    });

    g.bench_function(BenchmarkId::new("default_with_metadata", "builder"), |b| {
        b.iter(|| black_box(constructor::default_with_metadata()))
    });

    g.bench_function(BenchmarkId::new("default_with_metadata", "macro"), |b| {
        b.iter(|| black_box(constructor::default_with_metadata_macro()))
    });

    g.bench_function(
        BenchmarkId::new("default_with_metadata_source", "builder"),
        |b| b.iter(|| black_box(constructor::default_with_metadata_source())),
    );

    g.bench_function(
        BenchmarkId::new("default_with_metadata_source", "macro"),
        |b| b.iter(|| black_box(constructor::default_with_metadata_source_macro())),
    );

    g.finish();
}
