use std::fs;

pub fn read_input(year: usize, day: usize) -> String {
    let input_path = format!("input/year{}/day{:02}.txt", year, day);
    let input = fs::read_to_string(&input_path).expect("Could not find input file");

    input
}
