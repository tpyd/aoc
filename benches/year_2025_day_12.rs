use aoc::utils::read_input;
use criterion::{criterion_group, criterion_main, Criterion};
use aoc::year_2025_day_12;

fn bench_year_2025_day_12(c: &mut Criterion) {
    let mut group = c.benchmark_group("year_2025_day_12");
    let input = read_input(2025, 12);

    group.bench_function("solve", |b| b.iter(|| year_2025_day_12::run(&input)));

    group.finish();
}

criterion_group!(benches, bench_year_2025_day_12);
criterion_main!(benches);

