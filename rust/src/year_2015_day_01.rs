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
        let (floor, _) = run("(())"); 
        assert_eq!(floor, 0);

        let (floor, _) = run("()()"); 
        assert_eq!(floor, 0);

        let (floor, _) = run("((("); 
        assert_eq!(floor, 3);

        let (floor, _) = run("(()(()("); 
        assert_eq!(floor, 3);

        let (floor, _) = run("))((((("); 
        assert_eq!(floor, 3);

        let (floor, _) = run("())"); 
        assert_eq!(floor, -1);

        let (floor, _) = run("))("); 
        assert_eq!(floor, -1);

        let (floor, _) = run(")))"); 
        assert_eq!(floor, -3);

        let (floor, _) = run(")())())"); 
        assert_eq!(floor, -3);
    }    

    #[test]
    fn test_part2() {
        let (_, basement) = run(")"); 
        assert_eq!(basement, 1);

        let (_, basement) = run("()())"); 
        assert_eq!(basement, 5);
    }

    #[test]
    fn test_real() {
        let input = utils::read_input(2015, 1);
        let (floor, basement) = run(&input);

        assert_eq!(floor, 138);
        assert_eq!(basement, 1771);
    }
}

