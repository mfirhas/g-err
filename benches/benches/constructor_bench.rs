use std::hint::black_box;

use criterion::{BenchmarkId, Criterion};

#[path = "constructor.rs"]
mod constructor;

pub fn bench_all(c: &mut Criterion) {
    constructor_default_bench(c);
    constructor_with_config(c);
}

pub fn constructor_default_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("constructor");

    g.bench_function(BenchmarkId::new("default", "builder"), |b| {
        b.iter(|| black_box(constructor::default()))
    });

    g.bench_function(BenchmarkId::new("default", "macro"), |b| {
        b.iter(|| black_box(constructor::default_macro()))
    });

    g.bench_function(BenchmarkId::new("default_box", "builder"), |b| {
        b.iter(|| black_box(constructor::default_box()))
    });

    g.bench_function(BenchmarkId::new("default_box", "macro"), |b| {
        b.iter(|| black_box(constructor::default_box_macro()))
    });

    g.bench_function(BenchmarkId::new("default_fmt", "builder"), |b| {
        b.iter(|| black_box(constructor::default_fmt()))
    });

    g.bench_function(BenchmarkId::new("default_fmt", "macro"), |b| {
        b.iter(|| black_box(constructor::default_fmt_macro()))
    });

    g.bench_function(BenchmarkId::new("default_with_metadata", "builder"), |b| {
        b.iter(|| black_box(constructor::default_with_metadata()))
    });

    g.bench_function(BenchmarkId::new("default_with_metadata", "macro"), |b| {
        b.iter(|| black_box(constructor::default_with_metadata_macro()))
    });

    // -------------- anyhow --------------
    g.bench_function(BenchmarkId::new("anyhow", "builder"), |b| {
        b.iter(|| black_box(constructor::anyhow_builder()))
    });
    g.bench_function(BenchmarkId::new("anyhow", "macro"), |b| {
        b.iter(|| black_box(constructor::anyhow_macro()))
    });
    g.bench_function(BenchmarkId::new("anyhow_fmt", "builder"), |b| {
        b.iter(|| black_box(constructor::anyhow_fmt_builder()))
    });
    g.bench_function(BenchmarkId::new("anyhow_fmt", "macro"), |b| {
        b.iter(|| black_box(constructor::anyhow_fmt_macro()))
    });
    g.bench_function(BenchmarkId::new("anyhow_source", "builder"), |b| {
        b.iter(|| black_box(constructor::anyhow_source_builder()))
    });

    g.bench_function(BenchmarkId::new("anyhow_source", "macro"), |b| {
        b.iter(|| black_box(constructor::anyhow_source_macro()))
    });

    g.finish();
}

pub fn constructor_with_config(c: &mut Criterion) {
    let mut g = c.benchmark_group("constructor_config");

    g.bench_function(BenchmarkId::new("default_with_source", "builder"), |b| {
        b.iter(|| black_box(constructor::default_with_source()))
    });

    g.bench_function(BenchmarkId::new("default_with_source", "macro"), |b| {
        b.iter(|| black_box(constructor::default_with_source_macro()))
    });

    g.bench_function(
        BenchmarkId::new("default_with_metadata_source", "builder"),
        |b| b.iter(|| black_box(constructor::default_with_metadata_source())),
    );

    g.bench_function(
        BenchmarkId::new("default_with_metadata_source", "macro"),
        |b| b.iter(|| black_box(constructor::default_with_metadata_source_macro())),
    );

    // with config
    g.bench_function(BenchmarkId::new("config_manual_id", "builder"), |b| {
        b.iter(|| black_box(constructor::config_manual_id()))
    });
    g.bench_function(BenchmarkId::new("config_manual_id", "macro"), |b| {
        b.iter(|| black_box(constructor::config_manual_id_macro()))
    });
    g.bench_function(BenchmarkId::new("config_auto_id", "builder"), |b| {
        b.iter(|| black_box(constructor::config_auto_id()))
    });
    g.bench_function(BenchmarkId::new("config_auto_id", "macro"), |b| {
        b.iter(|| black_box(constructor::config_auto_id_macro()))
    });

    g.bench_function(
        BenchmarkId::new("config_manual_id_with_data", "builder"),
        |b| b.iter(|| black_box(constructor::config_manual_id_with_data())),
    );
    g.bench_function(
        BenchmarkId::new("config_manual_id_with_data", "macro"),
        |b| b.iter(|| black_box(constructor::config_manual_id_with_data_macro())),
    );
    g.bench_function(
        BenchmarkId::new("config_auto_id_with_data", "builder"),
        |b| b.iter(|| black_box(constructor::config_auto_id_with_data())),
    );
    g.bench_function(BenchmarkId::new("config_auto_id_with_data", "macro"), |b| {
        b.iter(|| black_box(constructor::config_auto_id_with_data_macro()))
    });

    g.finish();
}
