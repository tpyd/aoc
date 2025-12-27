fn get_movable(rolls: &[Vec<char>], movable: &mut Vec<(usize, usize)>) {
    let height = rolls.len();
    let width = rolls[0].len();

    for y in 0..height {
        for x in 0..width {
            if rolls[y][x] != '@' {
                continue;
            }

            let mut adjacent: u8 = 0;

            if y > 0 {
                // Top
                let cell = rolls[y - 1][x];  
                if cell == '@' {
                    adjacent += 1;
                }

                // Top-left
                if x > 0 {
                    let cell = rolls[y - 1][x - 1];  
                    if cell == '@' {
                        adjacent += 1;
                    }
                }

                // Top-right
                if x < width - 1 {
                    let cell = rolls[y - 1][x + 1];  
                    if cell == '@' {
                        adjacent += 1;
                    }
                }
            }

            // Left
            if x > 0 {
                let cell = rolls[y][x - 1];  
                if cell == '@' {
                    adjacent += 1;
                }
            }

            // Right
            if x < width - 1 {
                let cell = rolls[y][x + 1];  
                if cell == '@' {
                    adjacent += 1;
                }
            }

            if y < height - 1 {
                // Bottom
                let cell = rolls[y + 1][x];
                if cell == '@' {
                    adjacent += 1;
                }

                // Bottom-left
                if x > 0 {
                    let cell = rolls[y + 1][x - 1];  
                    if cell == '@' {
                        adjacent += 1;
                    }
                }


                // Bottom-right
                if x < width - 1 {
                    let cell = rolls[y + 1][x + 1];  
                    if cell == '@' {
                        adjacent += 1;
                    }
                }
            }

            if adjacent < 4 {
                movable.push((x, y));
            }
        }
    }
}

pub fn run(input: &str) -> (usize, usize) {
    let mut rolls = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut movable = Vec::new();
    let mut part1 = 0;
    let mut part2 = 0;

    loop {
        movable.clear();
        get_movable(&rolls, &mut movable);

        if movable.is_empty() {
            break;
        }

        if part1 == 0 {
            part1 = movable.len();
        }

        part2 += movable.len();

        for (x, y) in &movable {
            rolls[*y][*x] = '.';
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

