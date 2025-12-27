use std::{cmp::Reverse, collections::BinaryHeap};

fn find(i: usize, parents: &mut [usize]) -> usize {
    if i != parents[i] {
        let root = find(parents[i], parents);
        parents[i] = root;
    }

    parents[i]
}

pub fn run(input: &str) -> (u32, i64) {
    let num_connections = if input.len() < 1000 { 10 } else { 1000 };

    let boxes: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|junction_box| {
            let parts: Vec<&str> = junction_box.split(",").collect();

            let x = parts[0].parse::<i64>().unwrap();
            let y = parts[1].parse::<i64>().unwrap();
            let z = parts[2].parse::<i64>().unwrap();

            (x, y, z) 
        })
        .collect();

    let mut distances: BinaryHeap<Reverse<(i64, usize, usize)>> = BinaryHeap::new();

    for i in 0..boxes.len() - 1 {
        for j in i+1..boxes.len() {
            let (x1, y1, z1) = boxes[i]; 
            let (x2, y2, z2) = boxes[j]; 

            let distance = (x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2);

            distances.push(Reverse((distance, i, j)));
        }
    }

    let mut parents = Vec::new();
    for i in 0..boxes.len() {
        parents.push(i);
    }

    let mut sizes = vec![1u32; boxes.len()];

    let mut part1 = 0;
    let mut part2 = 0;

    let mut connections = 0;

    while let Some(Reverse((_, a, b))) = distances.pop() {
        connections += 1;

        let mut p1 = find(a, &mut parents);
        let mut p2 = find(b, &mut parents);

        if p1 == p2 {
            continue;
        }

        if sizes[p1] < sizes[p2] {
            std::mem::swap(&mut p1, &mut p2);
        }

        parents[p2] = parents[p1];
        sizes[p1] += sizes[p2];

        // Part 1
        if connections >= num_connections && part1 == 0 {
            let mut sizes_copy = sizes.clone();
            sizes_copy.sort_unstable_by(|a, b| b.cmp(a));

            part1 = sizes_copy
                .iter()
                .take(3)
                .product();
        }

        part2 = boxes[a].0 * boxes[b].0;

        if *sizes.iter().max().unwrap() as usize == boxes.len() {
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

