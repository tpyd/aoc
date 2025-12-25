use std::collections::{HashSet, VecDeque};

pub fn run(input: &str) -> (usize, usize) {
    let manifold: Vec<&str> = input
        .lines()
        .collect();

    let height = manifold.len();
    let width = manifold[0].len();

    let mut beams: VecDeque<(usize, usize, usize)> = VecDeque::new();
    beams.push_back((width / 2, 0, 1));

    let splitters: HashSet<(usize, usize)> = manifold[2..]
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line
                .chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    if c == '^' {
                        Some((x, y))
                    } else {
                        None
                    }
                })
        })
        .collect();

    let mut timelines = 0;
    let mut unique_splits = HashSet::new();

    while let Some((x, y, n)) = beams.pop_front() {
        if y >= height {
            timelines += n;
            continue;
        }

        if splitters.contains(&(x, y)) {
            unique_splits.insert((x, y));

            if let Some(idx) = beams.iter().position(|(fx, fy, _)| *fx == x + 1 && *fy == y + 2) {
                let (_, _, nn) = beams.remove(idx).unwrap();
                beams.push_back((x + 1, y + 2, n + nn));
            } else {
                beams.push_back((x + 1, y + 2, n));
            }

            if let Some(idx) = beams.iter().position(|(fx, fy, _)| *fx == x - 1 && *fy == y + 2) {
                let (_, _, nn) = beams.remove(idx).unwrap();
                beams.push_back((x - 1, y + 2, n + nn));
            } else {
                beams.push_back((x - 1, y + 2, n));
            }
        } else {
            beams.push_back((x, y + 2, n));
        }
    }

    (unique_splits.len(), timelines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_part1() {
        let (part1, _) = run(".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."); 
        assert_eq!(part1, 21);
    }    

    #[test]
    fn test_part2() {
        let (_, part2) = run(".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."); 
        assert_eq!(part2, 40);
    }

    #[test]
    fn test_real() {
        let input = utils::read_input(2025, 7);
        let (part1, part2) = run(&input);

        assert_eq!(part1, 1690);
        assert_eq!(part2, 221371496188107);
    }
}

