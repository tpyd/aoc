use core::panic;
use std::env;
use std::fs;

mod year2024 {
    pub mod day01;
}

use aoc::get_day;

fn read_input(day: u32, year: u32) -> String {
    let cwd = env::current_dir().unwrap();
    let filename = cwd
        .join("input")
        .join("2023")
        .join("real")
        .join(format!("day{day:02}.txt"));

    fs::read_to_string(filename)
        .expect("Could not find file {filename}")
        .trim()
        .to_string()
}

fn read_test_input(day: u32, part: Option<String>) -> String {
    let part = part.unwrap_or(String::from(""));
    let cwd = env::current_dir().unwrap();
    let filename = cwd
        .join("input")
        .join("2023")
        .join("test")
        .join(format!("day{day:02}{part}.txt"));

    fs::read_to_string(filename)
        .expect("Could not find file {filename}")
        .trim()
        .to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_str = &args[1];
    
    match day_str {
        "1" => => year2024::day01::run()
        _ => eprintln!("Could not run day {}", day_str)
    }

    return;

    let command = args[1].clone();
    let day_str = args[2].clone();

    // Example input can be different for part 1 and part 2.
    // Add option to add "part2" to use example input for part 2
    let test_part = args.get(3).map(|x| x.clone());
    if let Some(ref part_text) = test_part {
        if part_text != "part2" {
            panic!();
        }
    }

    let day: u32 = day_str.parse().expect("The day needs to be a number");

    let mut input = String::new();

    match command.as_str() {
        "run" => { input = read_input(day) },
        "test" => { input = read_test_input(day, test_part) },
        "bench" => unimplemented!(),
        "create" => unimplemented!(),
        "sync" => unimplemented!(),
        _ => panic!("Unknown command {command}"),
    }

    input = input.trim().replace("\r\n", "\n").to_string();

    get_day(day)(input);
}
