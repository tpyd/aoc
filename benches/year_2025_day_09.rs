use aoc::utils::read_input;
use criterion::{criterion_group, criterion_main, Criterion};
use aoc::year_2025_day_09;

fn bench_year_2025_day_09(c: &mut Criterion) {
    let mut group = c.benchmark_group("year_2025_day_09");
    let input = read_input(2025, 9);

    group.bench_function("solve", |b| b.iter(|| year_2025_day_09::run(&input)));

    group.finish();
}

criterion_group!(benches, bench_year_2025_day_09);
criterion_main!(benches);

