use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

#[derive(PartialEq, PartialOrd)]
struct Distance(f32, usize, usize);

impl Eq for Distance {}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

type MinDistance = Reverse<Distance>;

pub fn run(input: &str) -> (usize, i64) {
    let num_connections = if input.len() < 1000 { 10 } else { 1000 };

    let box_locations: Vec<(i64, i64, i64)> = input
        .trim_end()
        .split("\n")
        .map(|junction_box| {
            let parts: Vec<&str> = junction_box.split(",").collect();

            let x = parts[0].parse::<i64>().unwrap();
            let y = parts[1].parse::<i64>().unwrap();
            let z = parts[2].parse::<i64>().unwrap();

            (x, y, z) 
        })
        .collect();

    let mut distances: BinaryHeap<MinDistance> = BinaryHeap::new();

    for i in 0..box_locations.len() - 1 {
        for j in i+1..box_locations.len() {
            let (x1, y1, z1) = box_locations[i]; 
            let (x2, y2, z2) = box_locations[j]; 

            let distance = ((x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2)) as f32;

            distances.push(Reverse(Distance(distance, i, j)));
        }
    }

    let mut connections: Vec<HashSet<usize>> = Vec::new();
    for i in 0..box_locations.len() {
        let mut set = HashSet::new();
        set.insert(i);
        connections.push(set);
    }

    let mut part1 = 0;
    let mut part2 = 0;
    let mut counter = 0;

    loop {
        let Reverse(Distance(_, idx1, idx2)) = distances.pop().unwrap();

        let matches: Vec<usize> = connections
            .iter()
            .enumerate()
            .filter(|(_, s)| s.contains(&idx1) || s.contains(&idx2))
            .map(|(idx, _)| idx)
            .collect();
        
        // Merge if we have two different sets
        if matches.len() == 2 {
            let mut set1 = connections.remove(matches[0].max(matches[1]));
            let set2 = connections.remove(matches[0].min(matches[1]));
            set1.extend(&set2);
            connections.push(set1);

            if connections.len() == 1 && counter > num_connections {
                let (x1, _, _) = box_locations[idx1];
                let (x2, _, _) = box_locations[idx2];
                part2 = x1 * x2;
            }
        } else {
            let mut set = connections.remove(matches[0]);
            set.insert(idx1);
            set.insert(idx2);
            connections.push(set);
        }

        counter += 1;

        if counter == num_connections {
            connections.sort_unstable_by(|a, b| b.len().cmp(&a.len()));

            part1 = connections
                .iter()
                .take(3)
            .fold(1, |acc, s| acc * s.len());
        }

        if connections.len() == 1 && counter > num_connections {
            break; 
        }
    }

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_part1() {
        let (part1, _) = run("162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"); 
        assert_eq!(part1, 40);
    }    

    #[test]
    fn test_part2() {
        let (_, part2) = run("162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"); 
        assert_eq!(part2, 25272);
    }

    #[test]
    fn test_real() {
        let input = utils::read_input(2025, 8);
        let (part1, part2) = run(&input);

        assert_eq!(part1, 105952);
        assert_eq!(part2, 975931446);
    }
}

