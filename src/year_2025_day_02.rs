pub fn run(input: &str) -> (u64, u64) {
    let ids: Vec<u64> = input
        .trim_end()
        .split(",")
        .map(|x| {
            let (first, last) = x.split_once("-").unwrap();
            first.parse::<u64>().unwrap()..=last.parse::<u64>().unwrap()
        })
        .flatten()
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;

    for id in ids {
        let digits = id.ilog10() + 1;

        // Part 1
        if digits % 2 == 0 {
            let first = id / 10u64.pow(digits / 2);
            let second = id % 10u64.pow(digits / 2);

            if first == second {
                part1 += id;
            }
        }

        // Part 2
        for i in 1..=(digits / 2) {
            let pattern = id / 10u64.pow(digits - i);
            let pattern_len = pattern.ilog10() + 1;
            let mut repeated = pattern * 10u64.pow(pattern_len) + pattern;
            let mut repeated_len = repeated.ilog10() + 1;
            
            while repeated_len < digits {
                repeated = repeated * 10u64.pow(pattern_len) + pattern;
                repeated_len = repeated.ilog10() + 1;
            }
            
            if id == repeated {
                part2 += id;
                break;
            }
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
        let (part1, _) = run("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"); 
        assert_eq!(part1, 1227775554);
    }    

    #[test]
    fn test_part2() {
        let (_, part2) = run("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"); 
        assert_eq!(part2, 4174379265);
    }

    #[test]
    fn test_real() {
        let input = utils::read_input(2025, 2);
        let (part1, part2) = run(&input);

        assert_eq!(part1, 19128774598);
        assert_eq!(part2, 21932258645);
    }
}

