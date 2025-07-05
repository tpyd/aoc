use aoc::utils::read_input;
use criterion::{criterion_group, criterion_main, Criterion};
use aoc::year__day_;

fn bench_year__day_(c: &mut Criterion) {
    let mut group = c.benchmark_group("year__day_");
    let input = read_input(, );

    group.bench_function("solve", |b| b.iter(|| year__day_::run(&input)));

    group.finish();
}

criterion_group!(benches, bench_year__day_);
criterion_main!(benches);

