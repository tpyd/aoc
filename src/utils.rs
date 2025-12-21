use std::fs;

pub fn read_input(year: usize, day: usize) -> String {
    let input_path = format!("input/year_{year}_day_{day:02}.txt");
    let input = fs::read_to_string(&input_path).expect("Could not find input file");

    input
}
