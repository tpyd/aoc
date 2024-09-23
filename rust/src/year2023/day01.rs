use std::{arch::x86_64::_MM_FROUND_RAISE_EXC, collections::HashMap};

use itertools::Itertools;


fn part1(input: String) {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut sum = 0;

    for line in lines {
        let mut first_num = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                first_num = c.to_digit(10).unwrap();
                break;
            }
        }

        let mut second_num = 0;
        for c in line.chars().rev() {
            if c.is_digit(10) {
                second_num = c.to_digit(10).unwrap();
                break
            }
        }

        let num = first_num * 10 + second_num;
        sum += num;
    }
    
    println!("{sum}");
}

fn part2(input: String) {
    let words = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];

    let mut word_mappings = HashMap::new();
    word_mappings.insert("1", 1); 
    word_mappings.insert("2", 2); 
    word_mappings.insert("3", 3); 
    word_mappings.insert("4", 4); 
    word_mappings.insert("5", 5); 
    word_mappings.insert("6", 6); 
    word_mappings.insert("7", 7); 
    word_mappings.insert("8", 8); 
    word_mappings.insert("9", 9); 
    word_mappings.insert("one", 1); 
    word_mappings.insert("two", 2); 
    word_mappings.insert("three", 3); 
    word_mappings.insert("four", 4); 
    word_mappings.insert("five", 5); 
    word_mappings.insert("six", 6); 
    word_mappings.insert("seven", 7); 
    word_mappings.insert("eight", 8); 
    word_mappings.insert("nine", 9); 

    let mut sum = 0;

    for line in input.split('\n') {
        // First number
        let mut matches = vec![];

        for word in &words {
            let word_matches = line.match_indices(word).collect_vec();
            matches.extend(word_matches);
        }

        matches.sort_by(|a, b| a.0.cmp(&b.0));

        let mut first_num = 0;
        if matches.len() > 0 {
            let match_str = matches[0].1;
            first_num = match word_mappings.get(match_str) {
                Some(number) => *number,
                None => 0,
            };
        } 

        // Second number
        let mut matches = vec![];

        for word in &words {
            let word_matches = line.rmatch_indices(word).collect_vec();
            matches.extend(word_matches);
        }

        matches.sort_by(|a, b| a.0.cmp(&b.0));

        matches.reverse();

        let mut second_num = 0;
        if matches.len() > 0 {
            let match_str = matches[0].1;
            second_num = match word_mappings.get(match_str) {
                Some(number) => *number,
                None => 0,
            };
        }
       
        let num = first_num * 10 + second_num;
        sum += num;

    }

    println!("{sum}");
}

pub fn run(input: String) {
    part1(input.clone());
    part2(input);
}
