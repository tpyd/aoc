use aoc::utils::read_input;
use aoc::year_2025_day_02;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_year_2025_day_02(c: &mut Criterion) {
    let mut group = c.benchmark_group("year_2025_day_02");
    let input = read_input(2025, 2);

    group.bench_function("solve", |b| b.iter(|| year_2025_day_02::run(&input)));

    group.finish();
}

criterion_group!(benches, bench_year_2025_day_02);
criterion_main!(benches);
