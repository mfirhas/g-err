use std::hint::black_box;

use criterion::{Criterion, Throughput};
use g_err::{GErr, GErrDefault, gerr};

pub fn bench_all(c: &mut Criterion) {
    builder_bench(c);
    macro_bench(c);
}

const MSG: &str = "database connection failed";

pub fn builder_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("constructor/builder");

    g.throughput(Throughput::Bytes(MSG.len() as u64));
    g.bench_function("minimal", |b| b.iter(|| black_box(GErrDefault::new(MSG))));

    g.throughput(Throughput::Bytes(MSG.len() as u64));
    g.bench_function("full", |b| {
        b.iter(|| {
            black_box(
                GErr::<()>::new(MSG)
                    .set_code("DB-CONNECTION")
                    .add_tag("database")
                    .add_tag("critical")
                    .set_help("Verify database connectivity")
                    .with_data(("postgres", 5432_u16)),
            )
        })
    });

    g.finish();
}

pub fn macro_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("constructor/macro");

    g.throughput(Throughput::Bytes(MSG.len() as u64));
    g.bench_function("minimal", |b| b.iter(|| black_box(gerr!(MSG))));

    g.throughput(Throughput::Bytes("error: invalid: 123".len() as u64));
    g.bench_function("minimal_fmt", |b| {
        b.iter(|| black_box(gerr!("error: {}: {}", "invalid", 123)))
    });

    g.throughput(Throughput::Bytes(MSG.len() as u64));
    g.bench_function("full", |b| {
        b.iter(|| {
            black_box(gerr!(
                MSG;
                code = "DB-CONNECTION",
                tag = "database",
                tag = "critical",
                help = "Verify database connectivity",
                data = ("postgres", 5432_u16),
            ))
        })
    });

    g.finish();
}
