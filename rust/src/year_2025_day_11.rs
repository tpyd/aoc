use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

pub fn run(input: &str) -> (i32, u128) {
    let lines = input
        .trim_end()
        .split("\n");
    
    // TODO this can be &str i think?
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let parts: Vec<String> = line
            .replace(":", "")
            .split_whitespace()
            .map(|x| x.to_owned())
            .collect();
        let from = parts[0].to_owned();
        let to = &parts[1..];

        connections.insert(from, to.to_vec());
    }
    
    let mut to_check = vec!["you"];
    let mut part1 = 0;
    
    while let Some(device) = to_check.pop() {
        let new_devices = match connections.get(device) {
            Some(d) => d,
            None => continue
        };

        for new_device in new_devices {
            to_check.push(new_device);

            if new_device == "out" {
                part1 += 1;
            }
        }
    }

    // let start = "svr".to_owned();
    // let mut to_check: VecDeque<&str> = VecDeque::new();
    // to_check.push_back(&start);

    // let mut lookup: HashMap<&str, u128> = HashMap::new();
    // lookup.insert(&start, 1);
    //
    // let mut finish_layer = false;
    // let mut temp = 0;

    // 253057089600 too low

    // while let Some(device) = to_check.pop_front() {
    //     let new_devices = match connections.get(device) {
    //         Some(d) => d,
    //         None => continue
    //     };
    //
    //     let paths = *lookup.get(&device).unwrap();
    //
    //     // println!("currently at {device} with {paths} ways to get here");
    //
    //     if (device == "dac" || device == "fft") && finish_layer {
    //         dbg!(&device, &temp);
    //         let device_count = lookup.get_mut(device).unwrap();
    //         *device_count = temp;
    //         finish_layer = false;
    //         to_check.clear();
    //         to_check.push_back(device);
    //         lookup.clear();
    //         lookup.insert(device, temp);
    //         temp = 0;
    //         println!("Resetting from {device}");
    //         continue;
    //     };
    //
    //     for new_device in new_devices {
    //         match lookup.entry(&new_device) {
    //             Entry::Occupied(mut e) => {
    //                 // println!("Updating {new_device} paths");
    //                 e.insert(paths + e.get());
    //             },
    //             Entry::Vacant(e) => {
    //                 // println!("Adding {new_device} for the first time");
    //                 match new_device.as_str() {
    //                     "dac" | "fft" => {
    //                         dbg!("Found special");
    //                         temp += paths;
    //                         finish_layer = true;
    //                         e.insert(paths);
    //                     },
    //                     _ => {
    //                         e.insert(paths);
    //                     }
    //                 };
    //             }
    //         }
    //
    //         if !to_check.contains(&new_device.as_str()) {
    //             to_check.push_back(new_device);
    //         }
    //     }
    // }

    let start = "svr".to_owned();
    // let mut to_check: VecDeque<&str> = VecDeque::new();
    // to_check.push_back(&start);
    // let mut part2 = 0;

    fn runthing(start: String, end: String, connections: &HashMap<String, Vec<String>>) -> u128 {
        let mut to_check: VecDeque<&str> = VecDeque::new();
        to_check.push_back(&start);

        let mut lookup: HashMap<&str, u128> = HashMap::new();
        lookup.insert(&start, 1);

        while let Some(device) = to_check.pop_front() {
            let new_devices = match connections.get(device) {
                Some(d) => d,
                None => continue
            };

            let paths = *lookup.get(&device).unwrap();

            for new_device in new_devices {
                match lookup.entry(&new_device) {
                    Entry::Occupied(mut e) => {
                        e.insert(paths + e.get());
                    },
                    Entry::Vacant(e) => {
                        e.insert(paths);
                    }
                }

                if !to_check.contains(&new_device.as_str()) {
                    to_check.push_back(new_device);
                }
            }
        }

        *lookup.get("out").unwrap()
    }

    let a = runthing("svr".to_owned(), "fft".to_owned(), &connections);
    let b = runthing("fft".to_owned(), "dac".to_owned(), &connections);
    let c = runthing("dac".to_owned(), "out".to_owned(), &connections);
    dbg!(&a, &b, &c);
    let d = vec![a, b, c];
    let part2 = a.min(b.min(c));

    // dbg!(&lookup.keys().len());
    //
    // let part2 = *lookup.get("out").unwrap();
    // dbg!(&part2);
    // match device {
    //     "fft" => f |= 0b01,
    //     "dac" => f |= 0b10,
    //     _ => {}
    // }
    
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

//     #[test]
//     fn test_part1() {
//         let (part1, _) = run("aaa: you hhh
// you: bbb ccc
// bbb: ddd eee
// ccc: ddd eee fff
// ddd: ggg
// eee: out
// fff: out
// ggg: out
// hhh: ccc fff iii
// iii: out"); 
//         assert_eq!(part1, 5);
//     }    

    #[test]
    fn test_part2() {
        let (_, part2) = run("svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"); 
        assert_eq!(part2, 2);
    }

    // #[test]
    // fn test_real() {
    //     let input = utils::read_input(2025, 11);
    //     let (part1, part2) = run(&input);
    //
    //     assert_eq!(part1, 571);
    //     assert_eq!(part2, );
    // }
}

