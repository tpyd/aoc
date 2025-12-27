use aoc::utils::read_input;
use aoc::year_2015_day_01;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_year_2015_day_01(c: &mut Criterion) {
    let mut group = c.benchmark_group("year_2015_day_01");
    let input = read_input(2015, 1);

    group.bench_function("solve", |b| b.iter(|| year_2015_day_01::run(&input)));

    group.finish();
}

criterion_group!(benches, bench_year_2015_day_01);
criterion_main!(benches);
