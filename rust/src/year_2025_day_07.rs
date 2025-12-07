pub fn run(input: &str) -> (i32, usize) {
    let lines: Vec<&str> = input
        .trim_end()
        .split("\n")
        .collect();

    let height = lines.len();

    let mut beams = Vec::new(); 
    let mut quantum_beams = Vec::new(); 
    let mut splitters = Vec::new();
    let mut visited = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            match symbol {
                '^' => splitters.push((x, y)),
                'S' => {
                    beams.push((x, y));
                    quantum_beams.push((x, y, 1));
                },
                _ => {}
            }
        }
    }

    // TODO try hashmap/hashset for better perforamnce
    // TODO every second row is empty, can skip iterations
    let mut splits = 0;

    while let Some((x, y)) = beams.pop() {
        let ny = y + 1; 

        if ny >= height {
            continue;
        }

        if splitters.contains(&(x, ny)) {
            splits += 1;
            if !beams.contains(&(x+1, ny)) && !visited.contains(&(x, ny)) {
                beams.push((x+1, ny));
                visited.push((x+1, ny));
            }
            if !beams.contains(&(x-1, ny)) && !visited.contains(&(x, ny)) {
                beams.push((x-1, ny));
                visited.push((x-1, ny));
            }
        } else {
            if !beams.contains(&(x, ny)) && !visited.contains(&(x, ny)) {
                beams.push((x, ny));
                visited.push((x, ny));
            }
        }

    }

    let mut timelines = 0;

    while !quantum_beams.is_empty() {
        let (x, y, n) = quantum_beams.remove(0);
        let ny = y + 1;

        if ny >= height {
            timelines += n;
            continue;
        }

        if splitters.contains(&(x, ny)) {
            if quantum_beams.iter().any(|(ax, ay, _)| *ax == x+1 && *ay == ny) {
                let idx = quantum_beams.iter().position(|(fx, fy, _)| *fx == x+1 && *fy == ny).unwrap();
                let (_, _, nn) = quantum_beams.remove(idx);
                quantum_beams.push((x+1, ny, n + nn));
            } else {
                quantum_beams.push((x+1, ny, n));
            }
            if quantum_beams.iter().any(|(ax, ay, _)| *ax == x-1 && *ay == ny) {
                let idx = quantum_beams.iter().position(|(fx, fy, _)| *fx == x-1 && *fy == ny).unwrap();
                let (_, _, nn) = quantum_beams.remove(idx);
                quantum_beams.push((x-1, ny, n + nn));
            } else {
                quantum_beams.push((x-1, ny, n));
            }
        } else {
            quantum_beams.push((x, ny, n));
        }
    }

    (splits, timelines)
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

