use itertools::{Itertools, concat};

#[derive(Debug)]
enum Item {
    List(Vec<Item>),
    Number(i32),
    Empty,
}

pub fn part1(input: String) {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    let pairs = input
        .trim()
        .split("\n\n")
        .map(|x| {
            x.trim()
                .split('\n')
                .collect_tuple::<(&str, &str)>()  
                .unwrap()
        })
        .collect_vec();

    // Can easily be changed to iterators, but i cant debug, so solving it using loops
    for (a, b) in pairs {
        let item_a = get_item(a);
        dbg!(item_a);
    }

    println!("Part 1: {:?}", 0);
}

fn get_item(item_str: &str) -> String {
    let mut list_depth = 0;

    // Ignore first and last characters since its list branckets
    let s = &item_str.as_bytes()[1..];

    // Find first comma at the correct list level
    for (i, c) in s.iter().enumerate() {
        // Edge case where we reached the end, but didnt find a comma
        if *c as char == ']' && list_depth == 0 {
            return String::from("Empty list")
        }
        
        if *c as char == '[' {
            list_depth += 1;
            continue;
        }

        if *c as char == ']' {
            list_depth -= 1;
            continue;
        }

        // Found correct comma
        if *c as char == ',' && list_depth == 0 {
            let substr = &item_str[1..i+1];
            return String::from(substr);
        }
    }

    return String::from("ERROR")
}

pub fn part2(input: String) {

    println!("Part 2: {:?}", 0);
}
