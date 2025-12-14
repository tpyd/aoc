use std::collections::{HashMap, HashSet};

// TODO can we just do button indicies
// TODO can we just to button presses?
fn light_solutions(light_goal: &u32, buttons: Vec<Vec<u32>>) -> HashSet<Vec<usize>> {
    let light_start = 0u32;

    let max_len = 2usize.pow(buttons.len() as u32);
    let mut combinations: Vec<(u32, Vec<usize>)> = Vec::with_capacity(max_len);

    combinations.push((light_start.clone(), Vec::new()));

    let mut solutions = HashSet::new();

    for (i, button) in buttons.iter().enumerate() {
        // We either pressed it or not 
        let mut paths_copy = combinations.clone();

        for (lights, button_indicies) in &combinations {
            // Create a path where the button was pushed
            let mut new_lights = lights.clone();
            let mut new_indicies = button_indicies.clone();
            new_indicies.push(i);
            for b in button {
                new_lights ^= 1 << b;
            }

            if new_lights == *light_goal && !solutions.contains(&new_indicies) {
                solutions.insert(new_indicies.clone());
            }

            paths_copy.push((new_lights, new_indicies));

            // Create a path where the button was not pushed
            let lights2 = lights.clone();
            let buttons2 = button_indicies.clone();
            if !paths_copy.contains(&(lights.clone(), button_indicies.clone())) {
                paths_copy.push((lights2, buttons2.clone()));
            }

            if lights2 == *light_goal && !solutions.contains(&buttons2) {
                solutions.insert(buttons2);
            }
        }

        combinations = paths_copy;
    }

    solutions
}

fn find(
    joltages: Vec<u32>, 
    buttons: &Vec<Vec<u32>>, 
    cache: &mut HashMap<u32, HashSet<Vec<usize>>>
) -> u32 {
    if joltages.iter().all(|j| *j == 0) {
        return 0;
    }

    // Make it into a part 1 problem
    let mut lights = 0u32;  // TODO u16 faster?
    for (i, j) in joltages.iter().enumerate() {
        if j % 2 != 0 {
            lights |= 1 << i;
        }
    }
    // dbg!(format!("{:b}", lights));  // NOTE: its backwards
    
    // Find all the different ways to solve it. Returns indicies of buttons
    let ways_to_get_there: HashSet<Vec<usize>>;
    if cache.contains_key(&lights) {
        ways_to_get_there = cache.get(&lights).unwrap().clone();
    } else {
        ways_to_get_there = light_solutions(&lights, buttons.clone());
        cache.insert(lights, ways_to_get_there.clone());
    }

    let mut min_presses = 1000000;

    // Have to check every way. We can't just do minimal button presses, there could be
    // other paths to the correct answer with fewer button presses.
    'outer: for way in &ways_to_get_there {
        // Press the buttons, should end up with only even numbers
        let mut new_joltage = joltages.clone();
        for idx in way {
            let button = &buttons[*idx];
            for b in button {
                let joltage_value = new_joltage[*b as usize];
                if joltage_value == 0 {
                    // dbg!("nop");
                    continue 'outer;
                }
                new_joltage[*b as usize] -= 1;
            }

        }

        // Now we have only even numbers. Divide by 2
        for i in 0..new_joltage.len() {
            if new_joltage[i] % 2 != 0 {
                dbg!("WARNING");
            }
            new_joltage[i] /= 2;     
        }

        // Find the number of presses required to get here
        let presses_to_get_here = 2 * find(new_joltage, buttons, cache) + way.len() as u32;
        if presses_to_get_here < min_presses {  // TODO faster if we swap with .min()?
            min_presses = presses_to_get_here;
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
        let mut cache: HashMap<u32, HashSet<Vec<usize>>> = HashMap::new(); 
        let mut data = Vec::new();
        for i in joltage_goal {
            data.push(i as u32);
        }

        let button_presses = find(data, &buttons, &mut cache);
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

    // #[test]
    // fn test_real() {
    //     let input = utils::read_input(, );
    //     let (part1, part2) = run(&input);
    //
    //     assert_eq!(part1, 385);
    //     assert_eq!(part2, 16757);
    // }
}

