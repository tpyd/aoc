pub fn run(input: &str) -> (u64, i32) {
    let lines: Vec<Vec<String>> = input
        .trim_end()
        .split('\n')
        .map(|x| x.split_whitespace().map(|y| y.to_owned()).collect::<Vec<String>>())
        .collect();

    let mut sum = 0;
    let num_lines = lines.len();

    for col in 0..lines[0].len() {
        let mut values: Vec<u64> = Vec::new();

        let op = &lines[num_lines - 1][col];
        for line in 0..num_lines - 1 {
            let value = lines[line][col].parse::<u64>().unwrap();   
            values.push(value);
        }

        sum += match op.as_str() {
            "+" => values.iter().sum::<u64>(),
            _ => values.iter().product()
        };
    }

    (sum, 0)
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

    // #[test]
    // fn test_part2() {
    //     let (_, part2) = run("example"); 
    //     assert_eq!(part2, );
    // }
    //
    // #[test]
    // fn test_real() {
    //     let input = utils::read_input(, );
    //     let (part1, part2) = run(&input);
    //
    //     assert_eq!(part1, 4364617236318);
    //     assert_eq!(part2, );
    // }
}

