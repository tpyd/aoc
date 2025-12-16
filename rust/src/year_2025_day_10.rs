use std::collections::{hash_map::Entry, HashMap};

// TODO change to sum of buttons
// TODO does this need to be usize?
fn combinations(buttons: &[Vec<u32>], len: usize) -> HashMap<u32, Vec<(Vec<u32>, u32)>> {
    let max_len = 2usize.pow(buttons.len() as u32);
    let mut paths: Vec<(Vec<u32>, u32)> = Vec::with_capacity(max_len);
    let mut combinations: HashMap<u32, Vec<(Vec<u32>, u32)>> = HashMap::new();

    paths.push((vec![0; len], 0));

    for (i, button) in buttons.iter().enumerate() {
        // We either pressed it or not 
        let mut new_paths = paths.clone();

        for (acc_button, presses) in &paths {
            // Add a path where the button was pushed
            let mut new_acc_button = acc_button.clone();
            for j in button {
                new_acc_button[*j as usize] += 1; 
            }

            // Add both values to hashmap
            if i == buttons.len() - 1 {
                let mut parity_pressed = 0u32;
                for j in 0..new_acc_button.len() {
                    if new_acc_button[j] % 2 != 0 {
                        parity_pressed += 1 << j;
                    }
                }

                let mut parity_not_pressed = 0u32;
                for j in 0..acc_button.len() {
                    if acc_button[j] % 2 != 0 {
                        parity_not_pressed += 1 << j;
                    }
                }

                if (parity_pressed == 0 || parity_not_pressed == 0) && *presses == 3 {
                    // dbg!("her");
                    //
                    // dbg!(&presses, &button, &new_acc_button, &acc_button);
                }

                // TODO duplicates can occur, maybe filter out, can they??
                // println!("Adding {parity_pressed:b} with buttons {new_acc_button:?}");
                match combinations.entry(parity_pressed) {
                    Entry::Occupied(mut e) => {
                        let ref mut vec = *e.get_mut();
                        vec.push((new_acc_button, presses + 1));
                    },
                    Entry::Vacant(e) => {
                        e.insert(vec![(new_acc_button, presses + 1)]);
                    }
                }

                match combinations.entry(parity_not_pressed) {
                    Entry::Occupied(mut e) => {
                        let ref mut vec = *e.get_mut();
                        vec.push((acc_button.to_vec(), *presses));
                    },
                    Entry::Vacant(e) => {
                        e.insert(vec![(acc_button.to_vec(), *presses)]);
                    }
                }
            } else {
                new_paths.push((new_acc_button, presses + 1));
            }
        }

        paths = new_paths;

        // dbg!(&paths);
    }

    combinations
}

fn find(
    joltages: &[u32], 
    buttons: &Vec<Vec<u32>>,
    button_combinations: &HashMap<u32, Vec<(Vec<u32>, u32)>>,
    cache: &mut HashMap<Vec<u32>, u32>
) -> u32 {
    if joltages.iter().all(|j| *j == 0) {
        return 0;
    }

    let mut min_presses = 1000000;

    let mut parity = 0u32;
    for j in 0..joltages.len() {
        if joltages[j] % 2 != 0 {
            parity += 1 << j;
        }
    }

    let filtered_buttons = match button_combinations.get(&parity) {
        Some(b) => b,
        None => return min_presses
    };

    'outer: for (acc_button, combination_presses) in filtered_buttons {
        let mut new_joltages = joltages.to_vec();

        for (i, v) in acc_button.iter().enumerate() {
            let value = new_joltages[i];
            if *v > value {
                continue 'outer;
            }
            new_joltages[i] -= v; 
        }
        // dbg!(&new_joltages);

        for i in 0..new_joltages.len() {  // TODO merge with above loop
            new_joltages[i] /= 2;
        }

        let mut rec_presses = match cache.get(&new_joltages) {
            Some(&p) => p,
            None => {
                let p = find(&new_joltages, buttons, button_combinations, cache);
                cache.insert(new_joltages.to_vec(), p);
                p
            }
        };
        // let mut rec_presses = find(&new_joltages, buttons, button_combinations, cache);
        // dbg!(&new_joltages, &rec_presses, &min_presses);

        rec_presses = 2 * rec_presses + combination_presses;

        if rec_presses < min_presses {  // TODO faster if we swap with .min()?
            min_presses = rec_presses;
        }
    }

    min_presses
}

pub fn run(input: &str) -> (u32, u32) {
    let lines = input
        .trim_end()
        .split("\n");

    let mut part1 = 0;
    let mut part2 = 0;

    let length = lines.clone().count();

    for (round, line) in lines.enumerate() {
        let (lights_str, buttons_str) = line.split_once(" ").unwrap();
        
        let light_goal = lights_str
            .chars()
            .filter(|v| *v == '#' || *v == '.')
            .enumerate()
            .filter(|(_, v)| *v == '#')
            .fold(0, |acc, (i, _)| acc + (1 << i));

        let button_parts: Vec<&str> = buttons_str
            .split_whitespace()
            .collect();

        // TODO maybe parse as usize
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

        let mut lights = Vec::new();
        for _ in 0..lights_str.len() - 2 {
            lights.push(false);
        }

        let len = joltage_goal.len();
        let button_combinations = combinations(&buttons, len); 

        // Part 1
        part1 += button_combinations
            .get(&light_goal)
            .unwrap()
            .iter()
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .1;

        // Part 2
        let mut data = Vec::new();
        for i in &joltage_goal {
            data.push(*i as u32);
        }

        let button_presses = find(&data, &buttons, &button_combinations, &mut HashMap::new());
        part2 += button_presses;
    }

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_single() {
        let (part1, part2) = run("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(part1, 2);
        assert_eq!(part2, 10);
    }

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

