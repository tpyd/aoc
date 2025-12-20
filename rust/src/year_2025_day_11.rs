use core::panic;
use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

pub fn run(input: &str) -> (i32, u128) {
    let lines = input
        .trim_end()
        .split("\n");
    
    // TODO this can be &str i think?
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let parts: Vec<String> = line
            .replace(":", "")
            .split_whitespace()
            .map(|x| x.to_owned())
            .collect();
        let from = parts[0].to_owned();
        let to = &parts[1..];

        graph.insert(from, to.to_vec());
    }
    
    let mut to_check = vec!["you"];
    let mut part1 = 0;
    
    while let Some(device) = to_check.pop() {
        let new_devices = match graph.get(device) {
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

    // fn runthing(start: String, end: String, connections: &HashMap<String, Vec<String>>) -> u128 {
    //     let mut to_check: VecDeque<&str> = VecDeque::new();
    //     to_check.push_back(&start);
    //
    //     let mut lookup: HashMap<&str, u128> = HashMap::new();
    //     lookup.insert(&start, 1);
    //
    //     while let Some(device) = to_check.pop_front() {
    //         let new_devices = match connections.get(device) {
    //             Some(d) => d,
    //             None => continue
    //         };
    //
    //         let paths = *lookup.get(&device).unwrap();
    //
    //         for new_device in new_devices {
    //             match lookup.entry(&new_device) {
    //                 Entry::Occupied(mut e) => {
    //                     e.insert(paths + e.get());
    //                 },
    //                 Entry::Vacant(e) => {
    //                     e.insert(paths);
    //                 }
    //             }
    //
    //             if !to_check.contains(&new_device.as_str()) {
    //                 to_check.push_back(new_device);
    //             }
    //         }
    //     }
    //
    //     *lookup.get(end.as_str()).unwrap()
    // }
    //
    // let a = runthing("svr".to_owned(), "fft".to_owned(), &connections);
    // let b = runthing("fft".to_owned(), "dac".to_owned(), &connections);
    // let c = runthing("dac".to_owned(), "out".to_owned(), &connections);
    // dbg!(&a, &b, &c);
    // let d = vec![a, b, c];
    // let part2 = a * b * c;
    
    fn topological_sort(graph: &HashMap<String, Vec<String>>) -> Vec<String> {
        let mut indeg: HashMap<String, u32> = HashMap::new();
        
        for node in graph.keys() {
            indeg.insert(node.to_string(), 0);
        }

        for u in graph.keys() {
            for v in graph.get(u).unwrap() {
                match indeg.entry(v.to_string()) {
                    Entry::Occupied(mut e) => {
                        e.insert(e.get() + 1);
                    },
                    Entry::Vacant(e) => {
                        e.insert(1);
                    }
                }
            }
        }

        // dbg!(&indeg);

        let mut q: VecDeque<String> = VecDeque::new();
        let no_in = indeg.iter()
            .filter(|(_, v)| **v == 0)
            .map(|(k, _)| k);
        for k in no_in {
            q.push_back(k.to_string());
        }

        // dbg!(&q);

        let mut order: Vec<String> = Vec::new();

        while let Some(u) = q.pop_front() {
            order.push(u.clone());

            if graph.get(&u).is_none() {
                continue;
            }

            for v in graph.get(&u).unwrap() {
                match indeg.entry(v.to_string()) {
                    Entry::Occupied(mut e) => {
                        let new_val = e.get() - 1;
                        e.insert(new_val);
                        if new_val == 0 {
                            q.push_back(v.to_string());
                        }
                    },
                    Entry::Vacant(_) => panic!(),
                }
            }
        }

        // if graph.keys().count() != order.len() {
        //     dbg!(&graph, &order);
        //     panic!();
        // }

        // dbg!(&order);

        return order;
    }

    fn count_paths(
        graph: &HashMap<String, Vec<String>>, 
        from: String, 
        to: String, 
        topo_order: &Vec<String>
    ) -> u128 {
        let mut dp: HashMap<String, u128> = HashMap::new();
        
        for device in graph.keys() {
            dp.insert(device.to_string(), 0);
        }

        dp.insert(from, 1);
        dp.insert(to.clone(), 0);  // TODO maybe add when parsing input instead
        // dbg!(&dp);
                                   
        for u in topo_order {
            if *dp.get(u).unwrap() == 0 {
            //     // dbg!("no", &u);
                continue;
            }
            let u_val = dp.get(u).unwrap().clone();

            // Last element doesnt have entry in graph
            if graph.get(u).is_none() {
                continue;
            }

            for v in graph.get(u).unwrap() {
                // dbg!(&u, &v);
                match dp.entry(v.to_string()) {
                    Entry::Occupied(mut e) => {
                        e.insert(e.get() + u_val);
                    }
                    Entry::Vacant(_) => panic!("missing out value in dp")
                }
            }
        }

        // dbg!(&dp);
        *dp.get(&to).unwrap()
    }

    // TODO check if all explicit types are needed
    // TODO make owned string into references where possible
    // TODO change variable names to be more in line with the task
    // TODO lower u128 if possible
    // TODO can we filter topo_order before these calls?
    graph.insert("out".to_string(), Vec::new());
    let topo_order = topological_sort(&graph);
    let a = count_paths(&graph, "svr".to_string(), "fft".to_string(), &topo_order);
    let b = count_paths(&graph, "fft".to_string(), "dac".to_string(), &topo_order);
    let c = count_paths(&graph, "dac".to_string(), "out".to_string(), &topo_order);
    let part2 = a * b * c;
    
    // 511378159390560

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_part1() {
        let (part1, _) = run("aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"); 
        assert_eq!(part1, 5);
    }    

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

    #[test]
    fn test_real() {
        let input = utils::read_input(2025, 11);
        let (part1, part2) = run(&input);

        assert_eq!(part1, 571);
        assert_eq!(part2, 511378159390560);
    }
}

