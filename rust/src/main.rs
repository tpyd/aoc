mod utils;
mod year_2015_day_01;
mod year_2015_day_02;
mod year_2015_day_03;

fn print_solution<T: std::fmt::Display, U: std::fmt::Display>(solution: (T, U)) {
    let (part1, part2) = solution;
    println!("{}\n{}", part1, part2);
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
        (_, _) => println!("Could not find year/day combination")
    }
}

