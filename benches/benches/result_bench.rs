use criterion::Criterion;
use std::hint::black_box;

#[path = "result.rs"]
mod result;

pub fn bench_all(c: &mut Criterion) {
    result_simple_bench(c);
}

pub fn result_simple_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("result_simple");

    let input = "invalid_id";

    // -------------- g_err --------------
    g.bench_function("gerr_to_gerr", |b| {
        b.iter(|| black_box(result::g_err_bench::gerr_to_gerr(black_box(input)).unwrap_err()))
    });
    g.bench_function("gerr_context_auto", |b| {
        b.iter(|| black_box(result::g_err_bench::gerr_context_auto(black_box(input)).unwrap_err()))
    });
    g.bench_function("gerr_context", |b| {
        b.iter(|| black_box(result::g_err_bench::gerr_context(black_box(input)).unwrap_err()))
    });
    g.bench_function("gerr_wrap_err", |b| {
        b.iter(|| black_box(result::g_err_bench::gerr_wrap_err(black_box(input)).unwrap_err()))
    });
    // --
    g.bench_function("gerr_to", |b| {
        b.iter(|| black_box(result::g_err_bench::gerr_to(black_box(input)).unwrap_err()))
    });
    g.bench_function("gerr_gerr_auto", |b| {
        b.iter(|| black_box(result::g_err_bench::gerr_gerr_auto(black_box(input)).unwrap_err()))
    });
    g.bench_function("gerr_gerr", |b| {
        b.iter(|| black_box(result::g_err_bench::gerr_gerr(black_box(input)).unwrap_err()))
    });
    g.bench_function("gerr_wrap_gerr", |b| {
        b.iter(|| black_box(result::g_err_bench::gerr_wrap_gerr(black_box(input)).unwrap_err()))
    });
    g.bench_function("gerr_boxed", |b| {
        b.iter(|| black_box(result::g_err_bench::gerr_boxed(black_box(input)).unwrap_err()))
    });

    // -------------- snafu --------------
    g.bench_function("anyhow", |b| {
        b.iter(|| black_box(result::anyhow_bench::anyhow(black_box(input)).unwrap_err()))
    });
    g.bench_function("anyhow_bail", |b| {
        b.iter(|| black_box(result::anyhow_bench::anyhow_bail(black_box(input)).unwrap_err()))
    });
    g.bench_function("anyhow_context", |b| {
        b.iter(|| black_box(result::anyhow_bench::anyhow_context(black_box(input)).unwrap_err()))
    });
    g.bench_function("anyhow_context_with", |b| {
        b.iter(|| {
            black_box(result::anyhow_bench::anyhow_context_with(black_box(input)).unwrap_err())
        })
    });

    // -------------- snafu --------------
    g.bench_function("snafu_whatever", |b| {
        b.iter(|| black_box(result::snafu_bench::snafu_whatever(black_box(input)).unwrap_err()))
    });
    g.bench_function("snafu_whatever_source", |b| {
        b.iter(|| {
            black_box(result::snafu_bench::snafu_whatever_source(black_box(input)).unwrap_err())
        })
    });
    g.bench_function("snafu_whatever_macro", |b| {
        b.iter(|| {
            black_box(result::snafu_bench::snafu_whatever_macro(black_box(input)).unwrap_err())
        })
    });
    g.bench_function("snafu_whatever_source_macro", |b| {
        b.iter(|| {
            black_box(
                result::snafu_bench::snafu_whatever_source_macro(black_box(input)).unwrap_err(),
            )
        })
    });
    g.bench_function("snafu_whatever_macro_fmt", |b| {
        b.iter(|| {
            black_box(result::snafu_bench::snafu_whatever_macro_fmt(black_box(input)).unwrap_err())
        })
    });
    g.bench_function("snafu_whatever_source_macro_fmt", |b| {
        b.iter(|| {
            black_box(
                result::snafu_bench::snafu_whatever_source_macro_fmt(black_box(input)).unwrap_err(),
            )
        })
    });
    g.bench_function("snafu_whatever_context", |b| {
        b.iter(|| {
            black_box(result::snafu_bench::snafu_whatever_context(black_box(input)).unwrap_err())
        })
    });
    g.bench_function("snafu_whatever_context_with", |b| {
        b.iter(|| {
            black_box(
                result::snafu_bench::snafu_whatever_context_with(black_box(input)).unwrap_err(),
            )
        })
    });
    g.bench_function("snafu_context", |b| {
        b.iter(|| black_box(result::snafu_bench::snafu_context(black_box(input)).unwrap_err()))
    });
    g.bench_function("snafu_context_with", |b| {
        b.iter(|| black_box(result::snafu_bench::snafu_context_with(black_box(input)).unwrap_err()))
    });

    g.finish();
}
