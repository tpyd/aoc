use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
enum Turn {
    Santa,
    Robo
}

pub fn run(input: &str) -> (usize, usize) {
    let (mut part1_x, mut part1_y) = (0, 0);
    let (mut santa_x, mut santa_y) = (0, 0);
    let (mut robo_x, mut robo_y) = (0, 0);
    let mut turn = Turn::Santa;

    let mut part1_visited = HashSet::new();
    let mut part2_visited = HashSet::new();

    part1_visited.insert((0, 0));
    part2_visited.insert((0, 0));

    for d in input.chars() {
        let (mut dx, mut dy) = (0, 0);

        match d {
            '^' => dy = 1,
            'v' => dy = -1,
            '>' => dx = 1,
            '<' => dx = -1,
            _ => panic!()
        }

        (part1_x, part1_y) = (part1_x + dx, part1_y + dy);
        part1_visited.insert((part1_x, part1_y));

        if turn == Turn::Santa {
            (santa_x, santa_y) = (santa_x + dx, santa_y + dy);
            part2_visited.insert((santa_x, santa_y));
            turn = Turn::Robo;
        } else {
            (robo_x, robo_y) = (robo_x + dx, robo_y + dy);
            part2_visited.insert((robo_x, robo_y));
            turn = Turn::Santa;
        }
    }
    
    (part1_visited.len(), part2_visited.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_part1() {
        let (part1, _) = run(">"); 
        assert_eq!(part1, 2);

        let (part1, _) = run("^>v<"); 
        assert_eq!(part1, 4);

        let (part1, _) = run("^v^v^v^v^v"); 
        assert_eq!(part1, 2);
    }    

    #[test]
    fn test_part2() {
        let (_, part2) = run("^v"); 
        assert_eq!(part2, 3);

        let (_, part2) = run("^>v<"); 
        assert_eq!(part2, 3);

        let (_, part2) = run("^v^v^v^v^v"); 
        assert_eq!(part2, 11);
    }

    #[test]
    fn test_real() {
        let input = utils::read_input(2015, 3);
        let (part1, part2) = run(&input);

        assert_eq!(part1, 2592);
        assert_eq!(part2, 2360);
    }
}

