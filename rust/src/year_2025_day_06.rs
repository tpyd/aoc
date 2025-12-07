pub fn run(input: &str) -> (u64, u64) {
    let lines: Vec<Vec<char>> = input
        .trim_end_matches('\n')
        .split('\n')
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let width = lines[0].len();
    let height = lines.len();

    let mut horizontal_strings: Vec<String> = Vec::with_capacity(height - 1);
    for _ in 0..height - 1 {
        horizontal_strings.push(String::with_capacity(height - 1)); 
    }

    let mut vertical_values: Vec<u64> = Vec::with_capacity(height - 1);

    let mut operator = ' ';
    let mut vertical_string = String::with_capacity(height - 1);

    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    for col in 0..width {
        vertical_string.clear();

        for line in 0..height - 1 {
            let digit = lines[line][col];

            if digit != ' ' {
                horizontal_strings[line].push(digit);
                vertical_string.push(digit);
            }
        }

        // Empty column, calculate answer
        if vertical_string.is_empty() {
            let horizontal_values = horizontal_strings
                .iter()
                .map(|v| v.parse::<u64>().unwrap());

            sum_part1 += match operator {
                '+' => horizontal_values.sum::<u64>(),
                _ => horizontal_values.product()
            };

            sum_part2 += match operator {
                '+' => vertical_values.iter().sum::<u64>(),
                _ => vertical_values.iter().product()
            };

            for i in 0..height - 1 {
                horizontal_strings[i].clear();
            }
            vertical_values.clear();

            continue;
        }

        let value = vertical_string.parse::<u64>().unwrap(); 
        vertical_values.push(value);

        // Operator is always on the left
        if vertical_values.len() == 1 {
            operator = lines[height - 1][col]; 
        }
    }

    // Add last block
    let line_values = horizontal_strings
        .iter()
        .map(|v| v.parse::<u64>().unwrap());

    sum_part1 += match operator {
        '+' => line_values.sum::<u64>(),
        _ => line_values.product()
    };

    sum_part2 += match operator {
        '+' => vertical_values.iter().sum::<u64>(),
        _ => vertical_values.iter().product()
    };

    (sum_part1, sum_part2)
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

