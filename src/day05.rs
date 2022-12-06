use itertools::Itertools;

pub fn part1(input: String) {
    let (_, moves) = input
        .split("\n\n")
        .map(str::trim)
        .collect_tuple().unwrap();

    let mut stacks = vec![
        vec!['B', 'L', 'D', 'T', 'W', 'C', 'F', 'M'],
        vec!['N', 'B', 'L'],
        vec!['J', 'C', 'H', 'T', 'L', 'V'],
        vec!['S', 'P', 'J', 'W'],
        vec!['Z', 'S', 'C', 'F', 'T', 'L', 'R'],
        vec!['W', 'D', 'G', 'B', 'H', 'N', 'Z'],
        vec!['F', 'M', 'S', 'P', 'V', 'G', 'C', 'N'],
        vec!['W', 'Q', 'R', 'J', 'F', 'V', 'C', 'Z'],
        vec!['R', 'P', 'M', 'L', 'H']
    ];

    let instructions: Vec<(usize, usize, usize)> = moves
        .split('\n')
        .map(|line| {
            line
                .split(|x: char| !x.is_numeric())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize, usize)>().unwrap()
        })
        .collect();

    for (num, from, to) in instructions {
        for _ in 0..num {
            let ele = stacks[from-1].pop().unwrap();
            stacks[to-1].push(ele);
        }
    }

    let top_crates = stacks
        .iter()
        .map(|x| x.last().unwrap())
        .join("");

    println!("Part 1: {:?}", top_crates);
}

pub fn part2(input: String) {
    let sum = 0;

    println!("Part 2: {:?}", sum);
}