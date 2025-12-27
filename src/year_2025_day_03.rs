fn calculate_jolts(bank: &[u8], num_batteries: usize) -> u64 {
    let mut bank_joltage = 0;
    let mut index = 0;

    for n in 0..num_batteries {
        let mut max = bank[index];
        index += 1;
        
        let rest = bank
            .iter()
            .enumerate()
            .take(bank.len() - (num_batteries - 1 - n))
            .skip(index);

        for (i, &b) in rest {
            if b > max {
                max = b;
                index = i + 1;
            }
        }

        bank_joltage += (max - b'0') as u64 * 10u64.pow(num_batteries as u32 - 1 - n as u32);
    }
    
    bank_joltage
}

pub fn run(input: &str) -> (u64, u64) {
    input
        .lines()
        .map(|x| x.as_bytes())
        .fold((0, 0), |acc, b| (acc.0 + calculate_jolts(b, 2), acc.1 + calculate_jolts(b, 12)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_part1() {
        let (part1, _) = run("987654321111111
811111111111119
234234234234278
818181911112111"); 
        assert_eq!(part1, 357);
    }    

    #[test]
    fn test_part2() {
        let (_, part2) = run("987654321111111
811111111111119
234234234234278
818181911112111"); 
        assert_eq!(part2, 3121910778619);
    }

    #[test]
    fn test_real() {
        let input = utils::read_input(2025, 3);
        let (part1, part2) = run(&input);

        assert_eq!(part1, 17301);
        assert_eq!(part2, 172162399742349);
    }
}

