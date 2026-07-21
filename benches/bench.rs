use criterion::{Criterion, criterion_group, criterion_main};

mod benches {
    pub fn noop_bench(_: &mut criterion::Criterion) {}
}

fn criterion() -> Criterion {
    let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".bench");

    Criterion::default().output_directory(&output_dir)
}

criterion_group! {
    name = construction;
    config = criterion();
    targets = benches::noop_bench,
}

criterion_main!(construction,);
