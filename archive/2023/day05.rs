use std::ops::Range;

use itertools::Itertools;

// E.g seed to soil, soil to fertilizer etc.
#[derive(Debug)]
struct Step {
    transformations: Vec<Transformation>,
    mappings: Vec<Mapping>,
}

// One line in a step, e.g. 50 98 2
#[derive(Debug)]
struct Transformation {
    map_to: u64,
    map_start: u64,
    map_range: u64
}

#[derive(Debug)]
struct Mapping {
    range: Range<u64>,
    offset: u64
}

impl Step {
    fn transform(&self, pos: u64) -> u64 {
        for t in &self.transformations {
            if pos >= t.map_start && pos < t.map_start + t.map_range {
                let index = pos - t.map_start;
                return t.map_to + index;
            }
        }

        pos
    }

}

pub fn run(input: String) {
    let (seeds_str, input) = input.split_once("\n\n").unwrap();
    let (_, seeds_input) = seeds_str.split_once(':').unwrap();

    let seeds: Vec<u64> = seeds_input
        .trim()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();

    let groups = input.split("\n\n").collect_vec();

    let mut steps = Vec::new();
    
    for group in groups {
        let mut lines = group.split("\n").collect_vec();
        lines.remove(0);

        let mut transformations = Vec::new();
        let mut mappings = Vec::new();

        for line in lines {
            let (to, start, range) = line.split(' ').collect_tuple().unwrap();

            let map_to: u64 = to.parse().unwrap();
            let map_start: u64 = start.parse().unwrap();
            let map_range: u64 = range.parse().unwrap();
            
            let transformation = Transformation{ map_to, map_start, map_range };
            transformations.push(transformation);

            let map_range = Range{ start: map_start, end: map_start + map_range };
            let offset = 0;// map_to - map_start;
            
            let mapping = Mapping{ range: map_range, offset: offset };
            mappings.push(mapping);
        }

        transformations.sort_by(|a, b| a.map_start.cmp(&b.map_start));
        let step = Step{ transformations: transformations, mappings: mappings };
        steps.push(step);
    }

    let mut new_positions = Vec::new();

    for seed in &seeds {
        let mut position = seed.clone();

        for step in &steps {
            position = step.transform(position);
        }
        new_positions.push(position);
    }

    let ans = new_positions.iter().min().unwrap();

    println!("Part 1: {ans}");

    // Part 2

    // Parse seed ranges
    // let mut seed_ranges: Vec<Range<u64>> = Vec::new();
    // for chunk in &seeds.iter().chunks(2) {
    //     let (start, count) = chunk.collect_tuple().unwrap();
    //     seed_ranges.push(Range{ start: *start, end: start + count });
    // }

    // Make mappings
    // let mut mappings = Vec::new();

    // for step in steps {
    //     seed_ranges.sort_by(|a, b| a.start.cmp(&b.start));
        // let new_ranges = convert_input_to_output(seed_ranges, step);
        // let seed_ranges = new_ranges;
    // }

    let seed = Range{ start: 79, end: 79 + 14 }; // 93
    // let map = Range{ start: 98, end: 98 + 2 }; // 100
    let maps = vec![
        (Range{ start: 98, end: 98 + 2 }, 50 - 98), // 98
        (Range{ start: 50, end: 50 + 48 }, 52 - 50), // 98
    ];

    let mut new_seeds = Vec::new();
    for (map, offset) in maps {
        let new = convert(seed.clone(), map, offset);
        new_seeds.extend(new);
    }

    dbg!(&new_seeds);

    // let ans = new_positions.iter().min().unwrap();

    // println!("Part 2: {ans}");
}

fn convert<T: std::cmp::PartialOrd + std::ops::Add<Output = T> + Copy>(r1: Range<T>, r2: Range<T>, offset: T) -> Vec<Range<T>> {
    // If r1 is on the left side of r2 with no overlap
    if r1.end < r2.start {
        println!("r1 on left side of r2");
        return vec![r1];
    }

    // If r2 is on the left side of r2 with no overlap
    if r2.end < r1.end {
        println!("r2 on left side of r1");
        return vec![r1];
    }

    // If r1 is entirely inside r2
    if r1.start >= r2.start && r1.end <= r2.end {
        println!("r1 is inside r2");
        return vec![
            Range{ start: r1.start + offset, end: r1.end + offset },
        ];
    }

    // If r1 is entirely inside r2
    if r2.start >= r1.start && r2.end <= r1.end {
        println!("r1 is inside r2");
        unimplemented!();
    }

    // r1 overlaps on the left side
    if r1.start < r2.start && r1.end < r2.end && r1.end > r2.start {
        println!("r1 overlaps on the left side");
        return vec![
            Range{ start: r1.start, end: r2.start },
            Range{ start: r2.start + offset, end: r2.end + offset },
        ]
    }

    panic!("No condition met");
}