pub fn run(input: &str) -> (usize, usize) {
    let mut rolls = input
        .trim_end()
        .split('\n')
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let movable = get_movable(&rolls);
    let part1 = movable.len();

    let mut total_removed = 0;
    loop {
        let moved = get_movable(&rolls);

        if moved.len() == 0 {
            break;
        }

        total_removed += moved.len();

        for m in moved {
            rolls[m.1][m.0] = '.';
        }
    }

    (part1, total_removed)
}

fn get_movable(rolls: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let width = rolls[0].len();
    let height = rolls.len();

    let mut movable = Vec::new();

    // Center
    for y in 1..height-1 {
        for x in 1..width-1 {
            let cell = rolls[y][x];

            if cell != '@' {
                continue;
            }

            let top = &rolls[y-1][x-1..=x+1];
            let bottom = &rolls[y+1][x-1..=x+1];
            let left = &rolls[y][x-1];
            let right = &rolls[y][x+1];

            let mut counter = 0;
            counter += top.iter().filter(|x| **x == '@').count();
            counter += bottom.iter().filter(|x| **x == '@').count();
            counter += (*left == '@') as usize;
            counter += (*right == '@') as usize;
            
            if counter < 4 {
                movable.push((x, y));
            }
        }
    }

    // top row
    for x in 1..=width-2 {
        let cell = rolls[0][x];

        if cell != '@' {
            continue;
        }

        let bottom = &rolls[1][x-1..=x+1];
        let left = &rolls[0][x-1];
        let right = &rolls[0][x+1];

        let mut counter = 0;
        counter += bottom.iter().filter(|x| **x == '@').count();
        counter += (*left == '@') as usize;
        counter += (*right == '@') as usize;
        
        if counter < 4 {
            movable.push((x, 0));
        }
    }

    // bottom row
    for x in 1..width-1 {
        let cell = rolls[height-1][x];

        if cell != '@' {
            continue;
        }

        let top = &rolls[height-2][x-1..=x+1];
        let left = &rolls[height-1][x-1];
        let right = &rolls[height-1][x+1];

        let mut counter = 0;
        counter += top.iter().filter(|x| **x == '@').count();
        counter += (*left == '@') as usize;
        counter += (*right == '@') as usize;
        
        if counter < 4 {
            movable.push((x, height-1));
        }
    }

    // left row
    for y in 1..height-1 {
        let cell = rolls[y][0];

        if cell != '@' {
            continue;
        }

        let top = &rolls[y-1][0..=1];
        let bottom = &rolls[y+1][0..=1];
        let right = &rolls[y][1];

        let mut counter = 0;
        counter += top.iter().filter(|x| **x == '@').count();
        counter += bottom.iter().filter(|x| **x == '@').count();
        counter += (*right == '@') as usize;
        
        if counter < 4 {
            movable.push((0, y));
        }
    }

    // right row
    for y in 1..height-1 {
        let cell = rolls[y][width-1];

        if cell != '@' {
            continue;
        }

        let top = &rolls[y-1][width-2..=width-1];
        let bottom = &rolls[y+1][width-2..=width-1];
        let left = &rolls[y][width-2];

        let mut counter = 0;
        counter += top.iter().filter(|x| **x == '@').count();
        counter += bottom.iter().filter(|x| **x == '@').count();
        counter += (*left == '@') as usize;
        
        if counter < 4 {
            movable.push((width-1, y));
        }
    }

    // Corners
    if rolls[0][0] == '@' {
        let mut counter = 0;

        let bottom = &rolls[1][0..=1];
        let right = &rolls[0][1];
        
        counter += bottom.iter().filter(|x| **x == '@').count();
        counter += (*right == '@') as usize;

        if counter < 4 {
            movable.push((0, 0));
        }
    }

    if rolls[0][width-1] == '@' {
        let mut counter = 0;

        let bottom = &rolls[1][width-2..=width-1];
        let left = &rolls[0][width-2];
        
        counter += bottom.iter().filter(|x| **x == '@').count();
        counter += (*left == '@') as usize;

        if counter < 4 {
            movable.push((width-1, 0));
        }
    }

    if rolls[height-1][0] == '@' {
        let mut counter = 0;

        let top = &rolls[height-2][0..=1];
        let right = &rolls[height-1][1];
        
        counter += top.iter().filter(|x| **x == '@').count();
        counter += (*right == '@') as usize;

        if counter < 4 {
            movable.push((0, height-1));
        }
    }

    if rolls[height-1][width-1] == '@' {
        let mut counter = 0;

        let top = &rolls[height-1][width-2..=width-1];
        let left = &rolls[height-1][width-2];
        
        counter += top.iter().filter(|x| **x == '@').count();
        counter += (*left == '@') as usize;

        if counter < 4 {
            movable.push((width-1, height-1));
        }
    }

    movable
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_part1() {
        let (part1, _) = run("..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."); 
        assert_eq!(part1, 13);
    }    

    #[test]
    fn test_part2() {
        let (_, part2) = run("..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."); 
        assert_eq!(part2, 43);
    }

    #[test]
    fn test_real() {
        let input = utils::read_input(2025, 4);
        let (part1, part2) = run(&input);

        assert_eq!(part1, 1441);
        assert_eq!(part2, 9050);
    }
}

