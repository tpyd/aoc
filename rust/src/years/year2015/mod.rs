pub mod day01;
pub mod day02;

pub fn run_day(day: usize, input: &str) {
    match day {
        1 => print_results(day01::run(input)),
        2 => print_results(day02::run(input)),
        _ => println!("Could not find day {}", day),
    }
}

fn print_results<T: std::fmt::Display, U: std::fmt::Display>(results: (T, U)) {
    let (result1, result2) = results;
    
    println!("{}", result1);
    println!("{}", result2);
}
