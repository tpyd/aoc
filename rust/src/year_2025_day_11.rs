use core::panic;
use std::collections::{hash_map::Entry, HashMap, VecDeque};

pub fn run(input: &str) -> (u128, u128) {
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

    graph.insert("out".to_string(), Vec::new());
    
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

        let mut q: VecDeque<String> = VecDeque::new();
        let no_in = indeg.iter()
            .filter(|(_, v)| **v == 0)
            .map(|(k, _)| k);
        for k in no_in {
            q.push_back(k.to_string());
        }

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
        dp.insert(to.clone(), 0);  // Neccessary for part 1 test since it doesnt have fft or dac
                                   
        for u in topo_order {
            if *dp.get(u).unwrap() == 0 {
                continue;
            }
            let u_val = dp.get(u).unwrap().clone();

            for v in graph.get(u).unwrap() {
                match dp.entry(v.to_string()) {
                    Entry::Occupied(mut e) => {
                        e.insert(e.get() + u_val);
                    }
                    Entry::Vacant(_) => panic!("missing out value in dp")
                }
            }
        }

        *dp.get(&to).unwrap()
    }

    // TODO check if all explicit types are needed
    // TODO make owned string into references where possible
    // TODO change variable names to be more in line with the task
    // TODO lower u128 if possible
    // TODO can we filter topo_order before these calls?
    // TODO add possibility of dac before fft
    let topo_order = topological_sort(&graph);
    
    let part1 = count_paths(&graph, "you".to_string(), "out".to_string(), &topo_order);

    let a = count_paths(&graph, "svr".to_string(), "fft".to_string(), &topo_order);
    let b = count_paths(&graph, "fft".to_string(), "dac".to_string(), &topo_order);
    let c = count_paths(&graph, "dac".to_string(), "out".to_string(), &topo_order);
    let part2 = a * b * c;

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

