pub fn part1(input: String) {
    let sum: u32 = input
        .trim()
        .split('\n')
        .map(|x| {
            let (opponent, you): (&str, &str) = x.split_once(' ').unwrap();

            // A and X is rock, B and Y is paper, C and Z is scissors
            let mut points = match (opponent, you) {
                ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
                ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
                ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
                _ => panic!(),
            };

            // Rock is 1, Paper is 2, Scissors is 3
            points += match you {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!(),
            };

            points
        })
        .sum();

    println!("Part 1: {:?}", sum);
}

pub fn part2(input: String) {
    let sum: u32 = input
        .trim()
        .split('\n')
        .map(|x| {
            let (opponent, result): (&str, &str) = x.split_once(' ').unwrap();

            // A rock, B is paper, C is scissors
            // X is lose, Y is draw, Z is win
            let mut points = match result {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!(),
            };

            // Rock is 1, Paper is 2, Scissors is 3
            points += match (opponent, result) {
                ("A", "Y") | ("B", "X") | ("C", "Z") => 1,
                ("A", "Z") | ("B", "Y") | ("C", "X") => 2,
                ("B", "Z") | ("A", "X") | ("C", "Y") => 3,
                _ => panic!(),
            };

            points
        })
        .sum();

    println!("Part 2: {:?}", sum);
}
