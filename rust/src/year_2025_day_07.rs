pub fn run(input: &str) -> (i32, i32) {
    let lines: Vec<&str> = input
        .trim_end()
        .split("\n")
        .collect();

    let height = lines.len();

    let mut beams = Vec::new(); 
    let mut splitters = Vec::new();
    let mut visited = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            match symbol {
                '^' => splitters.push((x, y)),
                'S' => beams.push((x, y)),
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
            dbg!("spit at", &x, &ny);
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

    (splits, 0)
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
    //     assert_eq!(part1, );
    //     assert_eq!(part2, );
    // }
}

