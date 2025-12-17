use std::collections::HashMap;

pub fn run(input: &str) -> (usize, i32) {
    let lines = input
        .trim_end()
        .split("\n");
    
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
    let mut paths = 0;
    
    while let Some(device) = to_check.pop() {
        let new_devices = match connections.get(device) {
            Some(d) => d,
            None => continue
        };

        for new_device in new_devices {
            to_check.push(&new_device);

            if new_device == "out" {
                paths += 1;
            }
        }
    }

    (paths, 0)
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

    // #[test]
    // fn test_part2() {
    //     let (_, part2) = run("example"); 
    //     assert_eq!(part2, );
    // }
    //
    // #[test]
    // fn test_real() {
    //     let input = utils::read_input(2025, 11);
    //     let (part1, part2) = run(&input);
    //
    //     assert_eq!(part1, 571);
    //     assert_eq!(part2, );
    // }
}

