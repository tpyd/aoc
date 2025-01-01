use std::fs;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc::years::year2015::day01;

fn bench_day01(c: &mut Criterion) {
    let year = 2015;
    let day = 1;

    let input_path = format!("input/year{}/day{:02}.txt", year, day);
    let input = fs::read_to_string(&input_path).expect("Could not find input file");
        
    c.bench_function("run", |b| {
        b.iter(|| day01::run(black_box(&input)))
    });
}

criterion_group!(benches, bench_day01);
criterion_main!(benches);
