use std::collections::{HashMap, VecDeque};

fn paths(
    graph: &HashMap<&str, Vec<&str>>, 
    from: &str, 
    to: &str, 
    sorted: &[&str]
) -> u64 {
    let mut dp: HashMap<&str, u64> = graph
        .keys()
        .map(|&k| (k, 0))
        .collect();

    dp.insert(from, 1);
    dp.insert(to, 0);  // Neccessary for part 1 test since it doesn't have fft or dac
                               
    for u in sorted.iter().take_while(|x| **x != to) {
        let &u_val = dp.get(u).unwrap();

        if u_val == 0 {
            continue;
        }

        for v in graph.get(u).unwrap() {
            dp.entry(v)
                .and_modify(|e| *e += u_val);
        }
    }

    *dp.get(&to).unwrap()
}

pub fn run(input: &str) -> (u64, u64) {
    let lines = input
        .trim_end()
        .replace(":", "");
    
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut incoming: HashMap<&str, u32> = HashMap::new();

    for line in lines.split("\n") {
        let parts: Vec<&str> = line
            .split_whitespace()
            .collect();
        let u = parts[0];
        let v = &parts[1..];

        graph.insert(u, v.to_vec());
        incoming.insert(u, 0);
    }

    graph.insert("out", Vec::new());

    for &u in graph.keys() {
        for v in graph.get(u).unwrap() {
            incoming
                .entry(v)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }

    // Topological sort
    let mut queue: VecDeque<&str> = incoming.iter()
        .filter(|(_, v)| **v == 0)
        .map(|(&k, _)| k)
        .collect();

    let mut sorted: Vec<&str> = Vec::new();

    while let Some(u) = queue.pop_front() {
        sorted.push(u);

        let dest = graph.get(&u);
        if dest.is_none() {
            continue;
        }

        for v in dest.unwrap() {
            incoming
                .entry(v)
                .and_modify(|e| {
                    *e -= 1;
                    if *e == 0 {
                        queue.push_back(v); 
                    }
                });
        }
    }

    let part1 = paths(&graph, "you", "out", &sorted);

    let s2f = paths(&graph, "svr", "fft", &sorted);
    let s2d = paths(&graph, "svr", "dac", &sorted);

    let part2 = if s2f < s2d {
        let f2d = paths(&graph, "fft", "dac", &sorted);
        let d2o = paths(&graph, "dac", "out", &sorted);
        s2f * f2d * d2o
    } else {
        let d2f = paths(&graph, "dac", "fft", &sorted);
        let f2o = paths(&graph, "fft", "out", &sorted);
        s2d * d2f * f2o
    };

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

