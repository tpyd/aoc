use aoc::utils::read_input;
use criterion::{criterion_group, criterion_main, Criterion};
use aoc::year_2025_day_05;

fn bench_year_2025_day_05(c: &mut Criterion) {
    let mut group = c.benchmark_group("year_2025_day_05");
    let input = read_input(2025, 5);

    group.bench_function("solve", |b| b.iter(|| year_2025_day_05::run(&input)));

    group.finish();
}

criterion_group!(benches, bench_year_2025_day_05);
criterion_main!(benches);

