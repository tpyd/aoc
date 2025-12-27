use std::collections::{hash_map::Entry, HashMap};

fn combinations(
    buttons: &[Vec<usize>], 
    len: usize
) -> HashMap<u32, Vec<(Vec<i32>, u32)>> {
    let max_len = 2usize.pow(buttons.len() as u32);

    let mut paths: Vec<(Vec<i32>, u32)> = Vec::with_capacity(max_len);
    let mut combinations: HashMap<u32, Vec<(Vec<i32>, u32)>> = HashMap::new();

    paths.push((vec![0; len], 0));

    for (i, button) in buttons.iter().enumerate() {
        let mut new_paths = paths.clone();

        for (acc_button, presses) in &paths {
            let mut new_acc_button = acc_button.clone();
            for j in button {
                new_acc_button[*j] += 1; 
            }

            if i == buttons.len() - 1 {
                let mut parity_pressed = 0u32;
                for (j, new_acc) in new_acc_button.iter().enumerate() {
                    if new_acc % 2 != 0 {
                        parity_pressed += 1 << j;
                    }
                }

                let mut parity_not_pressed = 0u32;
                for (j, acc) in acc_button.iter().enumerate() {
                    if acc % 2 != 0 {
                        parity_not_pressed += 1 << j;
                    }
                }

                match combinations.entry(parity_pressed) {
                    Entry::Occupied(mut e) => {
                        e.get_mut().push((new_acc_button, presses + 1));
                    },
                    Entry::Vacant(e) => {
                        e.insert(vec![(new_acc_button, presses + 1)]);
                    }
                }

                match combinations.entry(parity_not_pressed) {
                    Entry::Occupied(mut e) => {
                        e.get_mut().push((acc_button.to_vec(), *presses));
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
    }

    combinations
}

fn find(
    joltages: &[i32], 
    button_combinations: &HashMap<u32, Vec<(Vec<i32>, u32)>>,
    cache: &mut HashMap<Vec<i32>, u32>
) -> u32 {
    if joltages.iter().all(|j| *j == 0) {
        return 0;
    }

    let mut min_presses = 1000000;

    let parity = joltages
        .iter()
        .enumerate()
        .filter(|(_, v)| *v % 2 != 0)
        .fold(0, |acc, (i, _)| acc + (1 << i));

    let filtered_buttons = match button_combinations.get(&parity) {
        Some(b) => b,
        None => return min_presses
    };

    for (acc_button, combination_presses) in filtered_buttons {
        let new_joltages: Vec<i32> = acc_button
            .iter()
            .zip(joltages)
            .map(|(acc, j)| (j - acc) / 2)
            .collect();

        if new_joltages.iter().any(|&j| j < 0) {
            continue;
        }

        let mut rec_presses = match cache.get(&new_joltages) {
            Some(&p) => p,
            None => {
                let p = find(&new_joltages, button_combinations, cache);
                cache.insert(new_joltages.to_vec(), p);
                p
            }
        };

        rec_presses = 2 * rec_presses + combination_presses;
        min_presses = min_presses.min(rec_presses);
    }

    min_presses
}

pub fn run(input: &str) -> (u32, u32) {
    let lines = input
        .trim_end()
        .split("\n");

    let mut part1 = 0;
    let mut part2 = 0;

    for line in lines {
        let (lights_str, rest) = line.split_once(" ").unwrap();
        let (buttons_str, joltages_str) = rest.rsplit_once(" ").unwrap();
        
        let light_goal = lights_str
            .chars()
            .filter(|v| *v == '#' || *v == '.')
            .enumerate()
            .filter(|(_, v)| *v == '#')
            .fold(0, |acc, (i, _)| acc + (1 << i));

        let buttons: Vec<Vec<usize>> = buttons_str
            .split_whitespace()
            .map(|button| button
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|n| n.to_digit(10).unwrap() as usize)
                .collect())
            .collect();

        let joltages: Vec<i32> = joltages_str
            .trim_matches(|c| c == '{' || c == '}')
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        let len = joltages.len();
        let button_combinations = combinations(&buttons, len); 

        part1 += button_combinations
            .get(&light_goal)
            .unwrap()
            .iter()
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .1;

        part2 += find(&joltages, &button_combinations, &mut HashMap::new());
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

