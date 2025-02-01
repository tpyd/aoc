mod utils;
mod years;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Usage: cargo run <year> <day>");
        std::process::exit(1);
    }

    let year = args[1].parse::<usize>().expect("Year must be a number");
    let day = args[2].parse::<usize>().expect("Day must be a number");

    let input = utils::get_input(year, day);

    match year {
        2015 => years::year2015::run_day(day, &input),
        _ => println!("Could not find year")
    }
}

