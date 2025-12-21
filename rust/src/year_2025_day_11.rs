use std::collections::{HashMap, VecDeque};

fn count_paths(
    graph: &HashMap<String, Vec<String>>, 
    from: String, 
    to: String, 
    topo_order: &Vec<String>
) -> u128 {
    let mut dp: HashMap<String, u128> = graph
        .keys()
        .map(|k| (k.to_string(), 0))
        .collect();

    dp.insert(from, 1);
    dp.insert(to.clone(), 0);  // Neccessary for part 1 test since it doesnt have fft or dac
                               
    for u in topo_order {
        let u_val = dp.get(u).unwrap().clone();

        if u_val == 0 {
            continue;
        }

        for v in graph.get(u).unwrap() {
            dp.entry(v.to_string())
                .and_modify(|e| *e = *e + u_val);
        }
    }

    *dp.get(&to).unwrap()
}

pub fn run(input: &str) -> (u128, u128) {
    let lines = input
        .trim_end()
        .split("\n");
    
    // TODO this (everything) can be &str i think?
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut incoming: HashMap<String, u32> = HashMap::new();

    for line in lines {
        let parts: Vec<String> = line
            .replace(":", "")
            .split_whitespace()
            .map(|x| x.to_owned())
            .collect();
        let from = parts[0].to_owned();
        let to = &parts[1..];

        graph.insert(from.to_string(), to.to_vec());
        incoming.insert(from, 0);
    }

    graph.insert("out".to_string(), Vec::new());

    for u in graph.keys() {
        for v in graph.get(u).unwrap() {
            incoming
                .entry(v.to_string())
                .and_modify(|e| *e = *e + 1)
                .or_insert(1);
        }
    }

    let mut queue: VecDeque<String> = incoming.iter()
        .filter(|(_, v)| **v == 0)
        .map(|(k, _)| k.to_string())
        .collect();

    let mut sorted: Vec<String> = Vec::new();

    while let Some(u) = queue.pop_front() {
        sorted.push(u.clone());

        let dest = graph.get(&u);
        if dest.is_none() {
            continue;
        }

        for v in dest.unwrap() {
            incoming
                .entry(v.to_string())
                .and_modify(|e| {
                    *e = *e - 1;
                    if *e == 0 {
                        queue.push_back(v.to_string()); 
                    }
                });
        }
    }

    // TODO check if all explicit types are needed
    // TODO make owned string into references where possible
    // TODO change variable names to be more in line with the task
    // TODO lower u128 if possible
    // TODO can we filter `sorted` before these calls to make it iterate less?
    // TODO add possibility of dac before fft
    let part1 = count_paths(&graph, "you".to_string(), "out".to_string(), &sorted);

    let a = count_paths(&graph, "svr".to_string(), "fft".to_string(), &sorted);
    let b = count_paths(&graph, "fft".to_string(), "dac".to_string(), &sorted);
    let c = count_paths(&graph, "dac".to_string(), "out".to_string(), &sorted);
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

