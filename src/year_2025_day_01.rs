pub fn run(input: &str) -> (i32, i32) {
    let rotations = input
        .trim_end()
        .split('\n')
        .into_iter()
        .map(|x| {
            let (dir, n) = x.split_at(1);
            (dir, n.parse::<i32>().unwrap())
        });

    let mut dial = 50;
    let mut zeros_part1 = 0;
    let mut zeros_part2 = 0;

    for (dir, n) in rotations {
        if n >= 100 {
            zeros_part2 += n / 100;
        }

        let rest = n % 100;

        match dir {
            "L" => {
                if rest >= dial && dial != 0 {
                    zeros_part2 += 1;
                }

                if rest > dial {
                    dial = 100 - (rest - dial)
                } else {
                    dial -= rest;
                }
            },
            _ => {
                if rest >= 100 - dial {
                    zeros_part2 += 1;
                }

                dial = (dial + rest) % 100;
            }
        }

        if dial == 0 {
            zeros_part1 += 1;
        }
    }
        
    (zeros_part1, zeros_part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_part1() {
        let (part1, _) = run("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"); 
        assert_eq!(part1, 3);
    }    

    #[test]
    fn test_part2() {
        let (_, part2) = run("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"); 
        assert_eq!(part2, 6);
    }

    #[test]
    fn test_real() {
        let input = utils::read_input(2025, 1);
        let (part1, part2) = run(&input);

        assert_eq!(part1, 1145);
        assert_eq!(part2, 6561);
    }
}

