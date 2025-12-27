use aoc::utils::read_input;
use aoc::year_2015_day_04;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_year_2015_day_04(c: &mut Criterion) {
    let mut group = c.benchmark_group("year_2015_day_04");
    let input = read_input(2015, 4);

    group.bench_function("solve", |b| b.iter(|| year_2015_day_04::run(&input)));

    group.finish();
}

criterion_group!(benches, bench_year_2015_day_04);
criterion_main!(benches);
