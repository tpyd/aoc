use std::collections::HashMap;

// TODO change to sum of buttons
fn combinations(buttons: &[Vec<u32>]) -> Vec<(Vec<usize>, u32)> {
    let max_len = 2usize.pow(buttons.len() as u32);
    // TODO change to hashmap
    let mut paths: Vec<(Vec<usize>, u32)> = Vec::with_capacity(max_len);

    paths.push((Vec::new(), 0));

    for (i, _) in buttons.iter().enumerate() {
        // We either pressed it or not 
        let mut new_paths = paths.clone();

        for (button_indicies, presses) in &paths {
            // Add a path where the button was pushed
            let mut pushed_indicies = button_indicies.clone();
            pushed_indicies.push(i);
            new_paths.push((pushed_indicies, presses + 1));
        }

        paths = new_paths;
    }

    paths
}

fn find(
    joltages: Vec<u32>, 
    buttons: &Vec<Vec<u32>>,
    button_combinations: &Vec<(Vec<usize>, u32)>,
    cache: &mut HashMap<Vec<u32>, u32>
) -> u32 {
    if joltages.iter().all(|j| *j == 0) {
        return 0;
    }

    let mut min_presses = 1000000;

    'outer: for (button_combination, combination_presses) in button_combinations {
        let mut new_joltage = joltages.clone();  

        for idx in button_combination {
            let button = &buttons[*idx];
            for b in button {
                let joltage_value = new_joltage[*b as usize];
                if joltage_value == 0 {
                    continue 'outer;
                }
                new_joltage[*b as usize] -= 1;
            }
        }

        for i in 0..new_joltage.len() {
            if new_joltage[i] % 2 != 0 {
                continue 'outer;
            }
            new_joltage[i] /= 2;
        }

        let mut presses;
        if cache.contains_key(&new_joltage) {
            presses = *cache.get(&new_joltage).unwrap();
        } else {
            presses = find(new_joltage.clone(), buttons, button_combinations, cache);
            cache.insert(new_joltage, presses);
        }

        presses = 2 * presses + combination_presses;

        if presses < min_presses {  // TODO faster if we swap with .min()?
            min_presses = presses;
        }
    }

    min_presses
}

pub fn run(input: &str) -> (i32, u32) {
    let lines = input
        .trim_end()
        .split("\n");

    let mut part1 = 0;
    let mut part2 = 0;

    let length = lines.clone().count();

    for (round, line) in lines.enumerate() {
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

        let mut buttons: Vec<Vec<u32>> = button_parts
            .clone()
            .into_iter()
            .take(button_parts.len() - 1)
            .map(|button| button
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|n| n.to_digit(10).unwrap())
                .collect())
            .collect();

        let joltage_str = button_parts
            .clone()
            .into_iter()
            .last()
            .unwrap();

        let mut joltage_goal: Vec<usize> = joltage_str[1..joltage_str.len()-1]
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        // dbg!(&joltage_requirements);

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
                for button in &buttons {
                    let mut new_lights = current_lights.clone();
                    let mut new_buttons = button_sequence.clone();

                    for b in button {
                        new_lights[*b as usize] ^= true;
                    }

                    if new_lights == light_goal {
                        part1 += button_presses;
                        break 'outer;
                    }

                    new_buttons.push(button);
                    possibilities.push((new_lights, new_buttons)); 
                }
            }
        }

        // Part 2
        let mut data = Vec::new();
        for i in joltage_goal {
            data.push(i as u32);
        }

        let button_combinations = combinations(&buttons);
        let mut cache = HashMap::new();

        let button_presses = find(data, &buttons, &button_combinations, &mut cache);
        part2 += button_presses;

        let iterstr = format!("Iteration {} / {}", round+1, length);
        dbg!(iterstr);
    }

    (part1, part2)
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

    #[test]
    fn test_part2() {
        let (_, part2) = run("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"); 
        assert_eq!(part2, 33);
    }

    #[test]
    fn test_real() {
        let input = utils::read_input(2025, 10);
        let (part1, part2) = run(&input);

        assert_eq!(part1, 385);
        assert_eq!(part2, 16757);
    }
}

