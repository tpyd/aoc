pub fn run(input: &str) -> (u64, u64) {
    let lines_part1: Vec<Vec<&str>> = input
        .trim_end()
        .split('\n')
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .collect();

    let width = lines_part1[0].len();
    let height = lines_part1.len();

    let mut sum = 0;
    let mut values: Vec<u64> = Vec::with_capacity(height - 1);

    for col in 0..width {
        values.clear();

        for line in 0..height - 1 {
            let value = lines_part1[line][col].parse::<u64>().unwrap();   
            values.push(value);
        }

        let operator = &lines_part1[height - 1][col];

        sum += match *operator {
            "+" => values.iter().sum::<u64>(),
            _ => values.iter().product()
        };
    }

    // Part 2
    let lines_part2: Vec<Vec<char>> = input
        .trim_end_matches('\n')
        .split('\n')
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    values.clear();

    let part2_width = lines_part2[0].len();
    let mut operator = ' ';
    let mut sum_part2 = 0;
    let mut value_string = String::with_capacity(height - 1);

    for col in 0..part2_width {
        value_string.clear();

        for line in 0..height - 1 {
            let digit = lines_part2[line][col];
            value_string.push(digit);
        }

        // Empty column, calculate answer
        if value_string.trim().is_empty() {
            sum_part2 += match operator {
                '+' => values.iter().sum::<u64>(),
                _ => values.iter().product()
            };

            values.clear();
            continue;
        }

        let value = value_string.trim().parse::<u64>().unwrap(); 
        values.push(value);

        // Operator is always on the left
        if values.len() == 1 {
            operator = lines_part2[height - 1][col]; 
        }
    }

    // Add last block
    sum_part2 += match operator {
        '+' => values.iter().sum::<u64>(),
        _ => values.iter().product()
    };

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

