pub fn run(input: &str) -> (i32, i32) {
    let rotations = input
        .lines()
        .map(|x| {
            let (dir, n) = x.split_at(1);
            (dir, n.parse::<i32>().unwrap())
        });

    let mut dial = 50;
    let mut part1 = 0;
    let mut part2 = 0;

    for (dir, n) in rotations {
        part2 += n / 100;
        let rest = n % 100;

        if dir == "L" {
            if rest >= dial && dial != 0 {
                part2 += 1;
            }

            if rest > dial {
                dial = 100 - (rest - dial)
            } else {
                dial -= rest;
            }
        } else {
            if rest >= 100 - dial {
                part2 += 1;
            }

            dial = (dial + rest) % 100;
        }

        if dial == 0 {
            part1 += 1;
        }
    }
        
    (part1, part2)
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

