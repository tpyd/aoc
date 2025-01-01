pub mod day01;

pub fn run_day(day: usize, input: &str) {
    match day {
        1 => print_results(day01::run(input)),
        _ => println!("Could not find day {}", day),
    }
}

fn print_results<T: std::fmt::Display, U: std::fmt::Display>(results: (T, U)) {
    let (result1, result2) = results;
    
    println!("{}", result1);
    println!("{}", result2);
}
