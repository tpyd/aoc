pub fn run(input: &str) -> (u64, u64) {
    let ranges = input
        .trim_end()
        .split(",")
        .map(|x| {
            let (first, last) = x.split_once("-").unwrap();
            first.parse::<u64>().unwrap()..=last.parse::<u64>().unwrap()
        });

    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
            
    for range in ranges {
        for n in range {
            let id = n.to_string();
            let digits = id.len();

            // Part 1
            let half = id.split_at(digits / 2).0;

            if id == half.repeat(2) {
                sum_part1 += n; 
            }

            // Part 2
            for i in 1..(digits / 2) + 1 {
                if digits % i != 0 {
                    continue;
                }

                let pattern = &id[..i];
                let repeated = pattern.repeat(digits / i); 

                if repeated == id {
                    sum_part2 += n;
                    break;
                }
            }
        }
    }

    return (sum_part1, sum_part2);
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

