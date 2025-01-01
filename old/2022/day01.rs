pub fn part1(input: String) {
    let calories: u32 = input.split("\n\n")
        .map(|elf| elf
            .trim()
            .split('\n')
            .map(|x| x.parse::<u32>().unwrap())
            .sum::<u32>()
        )
        .max().unwrap();

    println!("Part 1: {:?}", calories);
}

pub fn part2(input: String) {
    let mut elves: Vec<u32> = input.split("\n\n")
        .map(|elf| elf
            .trim()
            .split('\n')
            .map(|x| x.parse::<u32>().unwrap())
            .sum::<u32>()
        )
        .collect();

    elves.sort_unstable();

    let calories: u32 = elves
        .into_iter()
        .rev()
        .take(3)
        .sum();

    println!("Part 2: {:?}", calories);
}
