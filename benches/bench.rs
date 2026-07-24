use criterion::{Criterion, criterion_group, criterion_main};

mod benches {
    pub fn noop_bench(_: &mut criterion::Criterion) {}
    pub mod constructor_bench;
    pub mod result_bench;
}

fn criterion() -> Criterion {
    Criterion::default()
}

criterion_group! {
    name = construction;
    config = criterion();
    targets =
        benches::noop_bench,
        benches::constructor_bench::bench_all,
        benches::result_bench::bench_all,
}

criterion_main!(construction,);
