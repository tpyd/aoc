pub fn run(input: &str) -> (usize, u64) {
    let (ranges_raw, ids_raw) = input
        .trim_end()
        .split_once("\n\n")
        .unwrap();

    let mut ranges: Vec<(u64, u64)> = ranges_raw
        .lines()
        .map(|row| row
            .split_once("-")
            .map(|(first, last)| {
                let start = first.parse::<u64>().unwrap();
                let end = last.parse::<u64>().unwrap();
                (start, end)
            })
            .unwrap()
        )
        .collect();

    let ids: Vec<u64> = ids_raw
        .lines()
        .map(|id| id.parse::<u64>().unwrap())
        .collect();

    // Merge ranges
    let mut merged: Vec<(u64, u64)> = Vec::new();

    ranges.sort_unstable_by_key(|&(s, _)| s);

    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                if end > last.1 {
                    last.1 = end;
                }
                
                continue;
            }
        }

        merged.push((start, end));
    }

    let num_fresh = ids
        .iter()
        .filter(|id| merged.iter().any(|(start, end)| start <= id && *id <= end))
        .count();

    let num_possible_fresh = merged
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

