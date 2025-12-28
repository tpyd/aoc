use aoc::utils::read_input;
use aoc::year_2025_day_01;
use aoc::year_2025_day_02;
use aoc::year_2025_day_03;
use aoc::year_2025_day_04;
use aoc::year_2025_day_05;
use aoc::year_2025_day_06;
use aoc::year_2025_day_07;
use aoc::year_2025_day_08;
use aoc::year_2025_day_09;
use aoc::year_2025_day_10;
use aoc::year_2025_day_11;
use aoc::year_2025_day_12;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_year_2025(c: &mut Criterion) {
    let mut group = c.benchmark_group("year_2025");
    let input_day_1 = read_input(2025, 1);
    let input_day_2 = read_input(2025, 2);
    let input_day_3 = read_input(2025, 3);
    let input_day_4 = read_input(2025, 4);
    let input_day_5 = read_input(2025, 5);
    let input_day_6 = read_input(2025, 6);
    let input_day_7 = read_input(2025, 7);
    let input_day_8 = read_input(2025, 8);
    let input_day_9 = read_input(2025, 9);
    let input_day_10 = read_input(2025, 10);
    let input_day_11 = read_input(2025, 11);
    let input_day_12 = read_input(2025, 12);

    group.bench_function("solve_2025", |b| {
        b.iter(|| {
            std::hint::black_box(year_2025_day_01::run(&input_day_1));      
            std::hint::black_box(year_2025_day_02::run(&input_day_2));      
            std::hint::black_box(year_2025_day_03::run(&input_day_3));      
            std::hint::black_box(year_2025_day_04::run(&input_day_4));      
            std::hint::black_box(year_2025_day_05::run(&input_day_5));      
            std::hint::black_box(year_2025_day_06::run(&input_day_6));      
            std::hint::black_box(year_2025_day_07::run(&input_day_7));      
            std::hint::black_box(year_2025_day_08::run(&input_day_8));      
            std::hint::black_box(year_2025_day_09::run(&input_day_9));      
            std::hint::black_box(year_2025_day_10::run(&input_day_10));      
            std::hint::black_box(year_2025_day_11::run(&input_day_11));      
            std::hint::black_box(year_2025_day_12::run(&input_day_12));      
        });
    });

    group.finish();
}

criterion_group!(benches, bench_year_2025);
criterion_main!(benches);
