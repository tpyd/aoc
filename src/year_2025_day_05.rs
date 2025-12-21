pub fn run(input: &str) -> (usize, u64) {
    let (ranges_raw, ids_raw) = input
        .trim_end()
        .split_once("\n\n")
        .unwrap();

    let mut range_values: Vec<(u64, u64)> = ranges_raw
        .split("\n")
        .map(|row| row
            .split_once("-")
            .and_then(|(first, last)| {
                let start = first.parse::<u64>().unwrap();
                let end = last.parse::<u64>().unwrap();
                Some((start, end))
            })
            .unwrap()
        )
        .collect();

    range_values.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let ids: Vec<u64> = ids_raw
        .split("\n")
        .map(|id| id.parse::<u64>().unwrap())
        .collect();

    // Merge ranges
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    while range_values.len() >= 2 {
        let (start, mut end) = range_values.remove(0);

        loop {
            match range_values.iter().position(|(s, _)| *s <= end + 1) {
                Some(overlapping_range) => {
                    let overlapping_range = range_values.remove(overlapping_range);
                    let (_, e) = overlapping_range;

                    if e > end {
                        end = e; 
                    }
                },
                None => break
            }
        }

        ranges.push((start, end));
    }

    let num_fresh = ids
        .iter()
        .filter(|id| ranges.iter().any(|(start, end)| start <= id && *id <= end))
        .count();

    let num_possible_fresh = ranges
        .iter()
        .fold(0, |acc, (start, end)| acc + end - start + 1);

    (num_fresh, num_possible_fresh)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

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

