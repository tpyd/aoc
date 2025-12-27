pub fn run(input: &str) -> (i32, usize) {
    let mut floor = 0;
    let mut basement = 0;
    let mut found = false;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }

        if floor == -1 && !found {
            basement = i + 1;
            found = true
        }
    }

    (floor, basement)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_part1() {
        let (part1, _) = run("(())");
        assert_eq!(part1, 0);

        let (part1, _) = run("()()");
        assert_eq!(part1, 0);

        let (part1, _) = run("(((");
        assert_eq!(part1, 3);

        let (part1, _) = run("(()(()(");
        assert_eq!(part1, 3);

        let (part1, _) = run("))(((((");
        assert_eq!(part1, 3);

        let (part1, _) = run("())");
        assert_eq!(part1, -1);

        let (part1, _) = run("))(");
        assert_eq!(part1, -1);

        let (part1, _) = run(")))");
        assert_eq!(part1, -3);

        let (part1, _) = run(")())())");
        assert_eq!(part1, -3);
    }

    #[test]
    fn test_part2() {
        let (_, part2) = run(")");
        assert_eq!(part2, 1);

        let (_, part2) = run("()())");
        assert_eq!(part2, 5);
    }

    #[test]
    fn test_real() {
        let input = utils::read_input(2015, 1);
        let (part1, part2) = run(&input);

        assert_eq!(part1, 138);
        assert_eq!(part2, 1771);
    }
}
