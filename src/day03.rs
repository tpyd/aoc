use std::path::PrefixComponent;

use itertools::Itertools;
use regex::Regex;

pub fn part1(input: String) {
    let items = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let sum: u32 = input
    .trim()
    .split('\n')
        .map(|bag| {
            let mid = bag.len() / 2;
            let (a, b) = bag.split_at(mid);

            let item = a
                .chars()
                .filter(|x| b.contains(*x))
                .next().unwrap();

            let priority = items.find(item).unwrap() + 1;
            u32::try_from(priority).unwrap()
        })
        .sum();

    println!("The total sum of priorities are {:?}", sum);
}

pub fn part2(input: String) {
    let items = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    // Replace regex with vec chunks method if possible
    let re = Regex::new(r"([a-zA-Z]+\n){3}").unwrap();
    let sum: u32 = re.captures_iter(&input)
        .map(|group| {
            let bags: Vec<&str> = group
                .get(0).unwrap()
                .as_str()
                .trim()
                .split('\n')
                .collect();

            let (bag1, bag2, bag3) = bags.iter().collect_tuple().unwrap();
            let item = bag1
                .chars()
                .filter(|x| bag2.contains(*x))
                .filter(|x| bag3.contains(*x))
                .next().unwrap();

            let priority = items.find(item).unwrap() + 1;
            u32::try_from(priority).unwrap()
        })
        .sum();

    println!("The total sum of grouped priorities are {:?}", sum);
}