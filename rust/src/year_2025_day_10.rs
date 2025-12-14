use std::collections::{HashMap, VecDeque};

// TODO can we do bitmask?
// TODO can we just do button indicies
// TODO can we just to button presses?
fn possible_buttons(light_goal: &[bool], buttons: Vec<Vec<u32>>) -> Vec<Vec<usize>> {
    let light_start = vec![false; light_goal.len()];
    let mut paths = VecDeque::new();

    paths.push_back((light_start.clone(), Vec::new()));

    let mut yes = Vec::new();

    for (i, button) in buttons.iter().enumerate() {
        // We either pressed it or not 
        let mut paths_copy = paths.clone();
        for (lights, button_indicies) in &paths {
            // Create a path where the button was pushed
            let mut new_lights = lights.clone();
            let mut new_indicies = button_indicies.clone();
            new_indicies.push(i);
            for b in button {
                new_lights[*b as usize] ^= true;
            }

            paths_copy.push_back((new_lights, new_indicies));

            // Create a path where the button was not pushed
            let lights2 = lights.clone();
            let buttons2 = button_indicies.clone();
            if !paths_copy.contains(&(lights.clone(), button_indicies.clone())) {
                paths_copy.push_back((lights2, buttons2));
            }
        }
        paths = paths_copy;
    }

    // Check each path
    for (lights, button_idicies) in paths {
        if lights == light_goal {
            yes.push(button_idicies);
        }
    }

    // for a in &mut yes {
    //     a.sort_unstable();
    // }

    yes.sort_unstable();
    yes.dedup();
    yes
}

fn find(
    joltages: Vec<u32>, 
    buttons: &Vec<Vec<u32>>, 
    cache: &mut HashMap<Vec<bool>, Vec<Vec<usize>>>
) -> u32 {
    if joltages.iter().all(|j| *j == 0) {
        return 0;
    }

    // Make it into a part 1 problem
    let mut sub_ans = Vec::new();
    for j in &joltages {
        sub_ans.push(j % 2 != 0);
    }
    // true, true, false true
    
    // Find all the different ways to solve it. Returns indicies of buttons
    let mut ways_to_get_there: Vec<Vec<usize>>;
    if cache.contains_key(sub_ans.as_slice()) {
        // dbg!("cache hit");
        ways_to_get_there = cache.get(&sub_ans).unwrap().to_vec();
    } else {
        // dbg!("cache miss");
        ways_to_get_there = possible_buttons(&sub_ans, buttons.clone());
        // dbg!("calculated routes successfully");
        cache.insert(sub_ans.clone(), ways_to_get_there.clone());
    }

    // TODO do we need sort?
    ways_to_get_there.sort_unstable_by(|a, b| a.len().cmp(&b.len()));

    // Debug START
    let mut p1 = String::new(); 
    for s in sub_ans {
        p1.push(if s { '#' } else { '.' });
    }
    // dbg!(&joltages, &p1);
    for way in &ways_to_get_there {
        let mut debugbuttons = String::new();
        for idx in way {
            let button = &buttons[*idx];
            let mut b_string = String::new();
            b_string.push('(');
            for b in button {
                b_string.push_str(&b.to_string());
                b_string.push(',');
            }
            b_string.push_str(") ");
            debugbuttons.push_str(&b_string);
        }
        // dbg!(&debugbuttons);
    }
    // Debug END

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

    // let mut cache = HashMap::new();
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
        // dbg!(&light_goal, &buttons, &joltage_goal);
        let mut cache: HashMap<Vec<bool>, Vec<Vec<usize>>> = HashMap::new(); 

        let mut data = Vec::new();
        for i in joltage_goal {
            data.push(i as u32);
        }

        let button_presses = find(data, &buttons, &mut cache);
        // dbg!(&button_presses);
        part2 += button_presses;

        let iterstr = format!("Iteration {} / {}", round+1, length);
        dbg!(iterstr);

        //
        // goal = [3, 5, 4, 7]
        // a b0 = [0, 0, 0, 1]  max 7 times
        // b b1 = [0, 1, 0, 1]  max 5 times
        // c b2 = [0, 0, 1, 0]  max 4 times
        // d b3 = [0, 0, 1, 1]  max 4 times
        // e b4 = [1, 0, 1, 0]  max 3 times
        // f b5 = [1, 1, 0, 0]  max 3 times
        //
        // at least 3 button presses
        // at max 26 button presses?
        //
        //
        // minimize a + b + c + d + e + f  (lets call it x)
        // xb = [3,5,4,7]
        //
        // a * b0 + b * b1 + c * b2 + d * b3 + e * b4 + f * b5 = [3, 5, 4, 7]
        // e + f = 3
        // b + f = 5
        // c + d + e = 4
        // a + b + d = 7
        //
        //
        // Integer Linear Programming (ILP) problem
        // Solver is not that complicated. Complicated part is finding cuts to make it faster

        // Pure Python MILP solver
        // https://www.reddit.com/r/adventofcode/comments/1pity70/comment/nt988z4/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

        // Scipy with MILP
        // https://www.reddit.com/r/adventofcode/comments/1pity70/comment/nt9atam/
        
        // TODO i think the buttons are already gauss-elimiated, just needs sorting
        // minimize x  (sum of button presses)
        // Ax = b      (buttons * button presses = joltage goal)
        // x => 0      (button presses have to be positive)
        
        // m = 6, n = 4
        // button matrix m*n (6*4 matrix)
        // let mut button_vecs: Vec<Vec<i32>> = Vec::new();
        // let mut a = Vec::new();
        // for button in &buttons {
        //     let mut bits = 0i32;
        //     for i in button {
        //         bits |= 1 << i;
        //     }
        //     a.push(bits);
        //     // let mut vec = Vec::new();
        //     // for _ in 0..joltage_goal.len() {
        //     //     vec.push(0);
        //     // }
        //     // for i in button {
        //     //     vec[*i as usize] = 1;
        //     // }
        //     // button_vecs.push(vec);
        // }

        // dbg!(&a, &a.len());
        //
        // // b vector m (6)
        // let mut b = vec![0; joltage_goal.len()];
        // for i in 0..joltage_goal.len() {
        //     b[i] = joltage_goal[i] as i32;
        // }
        // dbg!(&b, &b.len());
        //
        // // variables, c or x vector n (4)
        // let mut joltages = Vec::new();
        // for _ in 0..buttons.len() {
        //     joltages.push(0.0);
        // }
        // let c = joltages;
        //
        // dbg!(&c, &c.len());

        // let constraints = build_constraints(&a, &b);
        // let presses = branch_and_bound(&constraints);
        // dbg!(&presses);

        // let (solution, obj) = simplex(&a, &b, &c).unwrap();
        // dbg!(&solution, &obj);


        // -------------------- DFS --------------------- //
        // let mut joltages = Vec::new();
        // for _ in 0..joltage_goal.len() {
        //     joltages.push(0);
        // }

        // Try buttons with many changes first
        // buttons.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
        // let mut total_button_presses = 999999;
        //
        // let mut stack = vec![(0, joltages)];
        //
        // 'outer: loop {
        //     if stack.len() == 0 {
        //         break;
        //     }
        //     let (previous_button_presses, previous_joltages) = stack.pop().unwrap();
        //
        //     // Try every button
        //     'nextbutton: for button in &buttons {
        //         let mut current_joltages = previous_joltages.clone();
        //         let next_button_presses = previous_button_presses + 1;
        //
        //         if next_button_presses >= total_button_presses {
        //             continue 'nextbutton;
        //         }
        //
        //         // Apply button
        //         for i in button {
        //             current_joltages[*i as usize] += 1;
        //         }
        //
        //         // Check if its the goal joltage
        //         if current_joltages == joltage_goal {
        //             if next_button_presses < total_button_presses {
        //                 total_button_presses = next_button_presses;
        //             }
        //             break 'outer;
        //         }
        //
        //         // Prune
        //         for i in 0..current_joltages.len() {
        //             if current_joltages[i] > joltage_goal[i] {
        //                 continue 'nextbutton;
        //             }
        //         }
        //
        //         stack.push((next_button_presses, current_joltages));
        //     }
        // }
        // dbg!("solved");
        // part2 += total_button_presses;
        // -------------------- DFS --------------------- //
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

