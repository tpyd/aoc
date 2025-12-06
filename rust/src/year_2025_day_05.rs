use std::ops::RangeInclusive;

pub fn run(input: &str) -> (u32, u64) {
    let (ranges, ids) = input
        .trim_end()
        .split_once("\n\n")
        .unwrap();

    let mut rangedvals: Vec<(u64, u64)> = Vec::new();
    for range in ranges.split('\n') {
        let (start, end) = range.split_once('-').unwrap();
        rangedvals.push((start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()));
    }

    rangedvals.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut new_ranges: Vec<RangeInclusive<u64>> = Vec::new();
    
    loop {
        if rangedvals.len() == 0 {
            break;
        }

        let (start, mut end) = rangedvals.remove(0);

        // Find overlapping ranges
        loop {
            let overlapping = rangedvals
                .iter()
                .position(|(s, e)| *s <= end + 1);

            if overlapping.is_none() {
                break;
            }

            let overlapping_range = rangedvals.remove(overlapping.unwrap());

            let (_, e) = overlapping_range;
            
            if e > end {
                end = e; 
            }
        }
            
        // let mut overlapping: Vec<usize> = rangedvals
        //     .iter()
        //     .enumerate()
        //     .filter(|(_, (s, e))| *s <= end + 1)  // TODO change to filter_map
        //     .map(|(idx, _)| idx)
        //     .collect();
        //
        // dbg!(&start, &end, &overlapping);
        //
        // loop {
        //     if overlapping.len() == 0 {
        //         break;
        //     }
        //
        //     let idx = overlapping.remove(0);
        //     let overlapping_range = rangedvals.remove(idx);
        //     let (_, e) = overlapping_range;
        //
        //     if e > end {
        //         end = e; 
        //
        //         overlapping = rangedvals
        //             .iter()
        //             .enumerate()
        //             .filter(|(_, (s, e))| *s <= end + 1)  // TODO change to filter_map
        //             .map(|(idx, _)| idx)
        //             .collect();
        //         dbg!("new", &overlapping);
        //     }
        // }

        new_ranges.push(start..=end);
    }


    let mut num_fresh = 0;
    for id in ids.split('\n') {
        let idnum = id.parse::<u64>().unwrap(); 

        for range in &new_ranges {
            if range.contains(&idnum) {
                num_fresh += 1; 
            }
        }
    }

    dbg!(&new_ranges);

    let mut possible_fresh = 0;
    for range in new_ranges {
        let count = range.count() as u64;
        possible_fresh += count;
        dbg!(&count);
    }

    (num_fresh, possible_fresh)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

//     #[test]
//     fn test_custom() {
//         let (_, part2) = run("3-5
// 3-5
// 10-20
// 6-7
//
// 1
// 5
// 8
// 11
// 17
// 32"); 
//         assert_eq!(part2, 15);
//     }

    #[test]
    fn test_part1() {
        let (part1, _) = run("3-5
10-14
16-20
12-18

1
5
8
11
17
32"); 
        assert_eq!(part1, 3);
    }    

    #[test]
    fn test_part2() {
        let (_, part2) = run("3-5
10-14
16-20
12-18

1
5
8
11
17
32"); 
        assert_eq!(part2, 14);
    }

    #[test]
    fn test_real() {
        let input = utils::read_input(2025, 5);
        let (part1, part2) = run(&input);

        assert_eq!(part1, 661);
        assert_eq!(part2, 359526404143208);
    }
}

