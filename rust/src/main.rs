mod years;

use std::fs;

fn main() {
    let year = 2015;
    let day = 1;

    let input_path = format!("input/year{}/day{:02}.txt", year, day);
    let input = fs::read_to_string(&input_path).expect("Could not find input file");

    match year {
        2015 => years::year2015::run_day(day, &input),
        _ => println!("Could not find year")
    }
}
