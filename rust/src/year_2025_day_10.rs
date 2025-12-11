pub fn run(input: &str) -> (i32, i32) {
    let lines = input
        .trim_end()
        .split("\n");

    let mut sum = 0;

    for line in lines {
        let (lights_str, buttons_str) = line.split_once(" ").unwrap();
        
        let num_lights = lights_str.len() - 2;
        let mut light_goal = Vec::new();
        for c in lights_str[1..lights_str.len() - 1].chars() {
            match c {
                '#' => light_goal.push(true),
                _ => light_goal.push(false)
            }
        }

        let button_parts: Vec<&str> = buttons_str
            .split_whitespace()
            .collect();

        let button_list: Vec<Vec<u32>> = button_parts
            .clone()
            .into_iter()
            .take(button_parts.len() - 1)
            .map(|button| button
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|n| n.to_digit(10).unwrap())
                .collect())
            .collect();

        let mut lights = Vec::new();
        for _ in 0..num_lights {
            lights.push(false);
        }

        // dbg!(&lights, &button_list, &light_goal);

        let mut possibilities = vec![(lights, vec![])];
        
        let mut button_presses = 0;
        'outer: loop {
            let possibilities_copy = possibilities.clone();
            possibilities.clear();

            button_presses += 1;

            for (current_lights, button_sequence) in possibilities_copy {
                for button in &button_list {
                    let mut new_lights = current_lights.clone();
                    let mut new_buttons = button_sequence.clone();

                    for b in button {
                        new_lights[*b as usize] ^= true;
                    }

                    if new_lights == light_goal {
                        sum += button_presses;
                        break 'outer;
                    }

                    new_buttons.push(button);
                    possibilities.push((new_lights, new_buttons)); 
                }
            }
        }
    }

    (sum, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_part1() {
        let (part1, _) = run("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"); 
        assert_eq!(part1, 7);
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

