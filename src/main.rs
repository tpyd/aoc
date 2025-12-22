mod utils;
mod year_2015_day_01;
mod year_2015_day_02;
mod year_2015_day_03;
mod year_2015_day_04;
mod year_2025_day_01;
mod year_2025_day_02;
mod year_2025_day_03;
mod year_2025_day_04;
mod year_2025_day_05;
mod year_2025_day_06;
mod year_2025_day_07;
mod year_2025_day_08;
mod year_2025_day_09;
mod year_2025_day_10;
mod year_2025_day_11;
mod year_2025_day_12;

fn print_solution<T: std::fmt::Debug, U: std::fmt::Debug>(solution: (T, U)) {
    let (part1, part2) = solution;
    println!("{:?}\n{:?}", part1, part2);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Usage: cargo run <year> <day>");
        std::process::exit(1);
    }

    let year = args[1].parse::<usize>().expect("Year must be a number");
    let day = args[2].parse::<usize>().expect("Day must be a number");

    let input = utils::read_input(year, day);

    match (year, day) {
        (2015, 1) => print_solution(year_2015_day_01::run(&input)),
        (2015, 2) => print_solution(year_2015_day_02::run(&input)),
        (2015, 3) => print_solution(year_2015_day_03::run(&input)),
        (2015, 4) => print_solution(year_2015_day_04::run(&input)),
        (2025, 1) => print_solution(year_2025_day_01::run(&input)),
        (2025, 2) => print_solution(year_2025_day_02::run(&input)),
        (2025, 3) => print_solution(year_2025_day_03::run(&input)),
        (2025, 4) => print_solution(year_2025_day_04::run(&input)),
        (2025, 5) => print_solution(year_2025_day_05::run(&input)),
        (2025, 6) => print_solution(year_2025_day_06::run(&input)),
        (2025, 7) => print_solution(year_2025_day_07::run(&input)),
        (2025, 8) => print_solution(year_2025_day_08::run(&input)),
        (2025, 9) => print_solution(year_2025_day_09::run(&input)),
        (2025, 10) => print_solution(year_2025_day_10::run(&input)),
        (2025, 11) => print_solution(year_2025_day_11::run(&input)),
        (2025, 12) => print_solution(year_2025_day_12::run(&input)),
        (_, _) => println!("Could not find year/day combination")
    }
}

