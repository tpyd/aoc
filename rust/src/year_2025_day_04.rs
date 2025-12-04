pub fn run(input: &str) -> (i32, i32) {
    let rolls = input
        .trim_end()
        .split('\n')
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let width = rolls[0].len();
    let height = rolls.len();

    let mut movable = 0;
    
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
                movable += 1; 
                dbg!("Found CENTER movable roll", &x, &y);
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
            movable += 1; 
            dbg!("Found TOP movable roll", &x, 0);
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
            movable += 1; 
            dbg!("Found BOTTOM movable roll", &x, &height);
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
            movable += 1; 
            dbg!("Found LEFT movable roll", 0, &y);
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
            movable += 1; 
            dbg!("Found RIGHT movable roll", &width, &y);
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
            movable += 1; 
            dbg!("Found TOP LEFT movable roll", 0, 0);
        }
    }

    if rolls[0][width-1] == '@' {
        let mut counter = 0;

        let bottom = &rolls[1][width-2..=width-1];
        let left = &rolls[0][width-2];
        
        counter += bottom.iter().filter(|x| **x == '@').count();
        counter += (*left == '@') as usize;

        if counter < 4 {
            movable += 1; 
            dbg!("Found TOP RIGHT movable roll", &width, 0);
        }
    }

    if rolls[height-1][0] == '@' {
        let mut counter = 0;

        let top = &rolls[height-2][0..=1];
        let right = &rolls[height-1][1];
        
        counter += top.iter().filter(|x| **x == '@').count();
        counter += (*right == '@') as usize;

        if counter < 4 {
            movable += 1;
            dbg!("Found BOTTOM LEFT movable roll", 0, &height);
        }
    }

    if rolls[height-1][width-1] == '@' {
        let mut counter = 0;

        let top = &rolls[height-1][width-2..=width-1];
        let left = &rolls[height-1][width-2];
        
        counter += top.iter().filter(|x| **x == '@').count();
        counter += (*left == '@') as usize;

        if counter < 4 {
            movable += 1; 
            dbg!("Found BOTTOM RIGHT movable roll", &width, &height);
        }
    }

    (movable, 0)
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

