pub fn run(input: &str) -> (u64, u64) {
    let banks = input
        .trim_end()
        .split('\n')
        .map(|x| x.as_bytes());

    let mut b1;
    let mut b2;
    let mut output_joltage = 0;
    let mut output_joltage_part2 = 0;

    for bank in banks {
        b1 = bank[0];
        b2 = bank[1];

        for battery in &bank[2..] {
            if b2 > b1 {
                b1 = b2;
                b2 = *battery;
                continue;
            }

            if *battery > b2 {
                b2 = *battery;
            }
        }

        let jolts = ((b1 - 48).to_string() + &(b2 - 48).to_string()).parse::<u64>().unwrap();
        output_joltage += jolts;

        // Part 2
        let mut bank_joltage = 0;

        let mut index = 0;
        for i in 0..12 {
            let mut max = bank[index];
            index += 1;
            for j in index..bank.len()-(11-i) {
                if bank[j] > max {
                    max = bank[j];
                    index = j + 1;
                }
            }
            bank_joltage += (max - 48) as u64 * 10u64.pow(11 - i as u32);
        }

        output_joltage_part2 += bank_joltage;
    }

    (output_joltage, output_joltage_part2)
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

