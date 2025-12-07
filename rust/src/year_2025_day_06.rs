pub fn run(input: &str) -> (u64, u64) {
    let lines: Vec<Vec<&str>> = input
        .trim_end()
        .split('\n')
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .collect();

    let mut sum = 0;
    let num_lines = lines.len();
    let width = lines[0].len();
    let mut values: Vec<u64> = Vec::with_capacity(num_lines);

    for col in 0..width {
        values.clear();

        for line in 0..num_lines - 1 {
            let value = lines[line][col].parse::<u64>().unwrap();   
            values.push(value);
        }

        let operator = &lines[num_lines - 1][col];

        sum += match *operator {
            "+" => values.iter().sum::<u64>(),
            _ => values.iter().product()
        };
    }

    // Part 2
    let lines2: Vec<Vec<char>> = input
        .trim_end_matches('\n')
        .split('\n')
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let mut newlines: Vec<String> = Vec::new();

    // Transpose input
    let width = lines2[0].len();
    for col in (0..width).rev() {
        let mut actual = String::with_capacity(width);
        for row in 0..lines2.len() {
            actual.push(lines2[row][col]);
        }
        newlines.push(actual); 
    }

    // Calculate sum
    let groups = newlines.iter().map(|x| x.trim()).collect::<Vec<&str>>().join("\n");
    let problems = groups
        .split("\n\n")
        .map(|x| x.split('\n').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut sum_part2 = 0;
    let new_num_lines = problems[0].len();
    for problem in problems {
        let op = problem[problem.len()-1].bytes().last().unwrap() as char;

        let mut values: Vec<u64> = Vec::new();
        for (idx, val) in problem.iter().enumerate() {
            if val.chars().last().unwrap().is_digit(10) {
                values.push(val.parse::<u64>().unwrap());
            } else {
                values.push(val[..val.len()-1].trim().parse::<u64>().unwrap());
            }
        }

        // dbg!("a", &values, &op);
        sum_part2 += match op.to_string().as_str() {
            "+" => values.iter().sum::<u64>(),
            _ => values.iter().product()
        };
    }

    (sum, sum_part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_part1() {
        let (part1, _) = run("123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "); 
        assert_eq!(part1, 4277556);
    }    

    #[test]
    fn test_part2() {
        let (_, part2) = run("123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "); 
        assert_eq!(part2, 3263827);
    }

    #[test]
    fn test_real() {
        let input = utils::read_input(2025, 6);
        let (part1, part2) = run(&input);

        assert_eq!(part1, 4364617236318);
        assert_eq!(part2, 9077004354241);
    }
}

