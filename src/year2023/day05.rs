use itertools::Itertools;

// E.g seed to soil, soil to fertilizer etc.
#[derive(Debug)]
struct Step {
    transformations: Vec<Transformation>,
}

// One line in a step, e.g. 50 98 2
#[derive(Debug)]
struct Transformation {
    map_to: u64,
    map_start: u64,
    map_range: u64
}

impl Step {
    fn transform(&self, pos: u64) -> u64 {
        for t in &self.transformations {
            if pos >= t.map_start && pos < t.map_start + t.map_range {
                let index = pos - t.map_start;
                return t.map_to + index;
            }

            // if let Some(index) = (t.map_start..t.map_start+t.map_range).position(|x| x == pos) {
            //     return t.map_to + index as u32;
            // }
        }

        pos
    }
}

pub fn run(input: String) {
    let (seeds_str, input) = input.split_once("\n\n").unwrap();
    let (_, seeds_input) = seeds_str.split_once(':').unwrap();

    let seeds = seeds_input.trim().split(' ').map(|x| x.parse::<u64>().unwrap()).collect_vec();

    let groups = input.split("\n\n").collect_vec();

    let mut steps = Vec::new();
    
    for group in groups {
        let mut lines = group.split("\n").collect_vec();
        lines.remove(0);

        let mut transformations = Vec::new();

        for line in lines {
            let (to, start, range) = line.split(' ').collect_tuple().unwrap();

            let map_to: u64 = to.parse().unwrap();
            let map_start: u64 = start.parse().unwrap();
            let map_range: u64 = range.parse().unwrap();
            
            let transformation = Transformation{ map_to, map_start, map_range };
            transformations.push(transformation);
        }

        let step = Step{ transformations: transformations };
        steps.push(step);
    }

    let mut new_positions = Vec::new();

    for seed in seeds {
        let mut position = seed;

        for step in &steps {
            position = step.transform(position);
        }
        new_positions.push(position);
    }

    let ans = new_positions.iter().min().unwrap();

    println!("Part 1: {ans}");
}
