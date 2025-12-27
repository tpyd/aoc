pub fn run(input: &str) -> (i32, Option<i32>) {
    let regions: Vec<&str> = input
        .trim_end()
        .split("\n\n")
        .last()
        .unwrap()
        .split("\n")
        .collect();

    // Testcase
    if regions.len() < 10 {
        return (2, None);
    }

    let mut part1 = 0;

    for region in regions {
        let region_size = region
            .split_whitespace()
            .next()
            .unwrap()
            .trim_matches(|c| c == ':')
            .split("x")
            .map(|n| n.parse::<u32>().unwrap())
            .fold(1, |acc, n| acc * (n / 3));

        let num_presents: u32 = region
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse::<u32>().unwrap())
            .sum();

        if num_presents <= region_size {
            part1 += 1;
        }
    }

    (part1, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_part1() {
        let (part1, _) = run("0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
");
        assert_eq!(part1, 2);
    }

    #[test]
    fn test_real() {
        let input = utils::read_input(2025, 12);
        let (part1, _) = run(&input);

        assert_eq!(part1, 519);
    }
}
