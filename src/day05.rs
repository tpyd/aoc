use itertools::Itertools;

pub fn part1(input: String) {
    let (crates, moves) = input
        .split("\n\n")
        .map(str::trim)
        .collect_tuple().unwrap();

    let num_stacks = crates
        .split('\n')
        .next_back().unwrap()
        .split(|x| !char::is_numeric(x))
        .map(|x| {println!("{:?}", x); 1})
        // .map(|x| x.parse::<u32>().unwrap())
        .max().unwrap();


    println!("{:?}", num_stacks);


    println!("Part 1: {:?}", 0);
}

pub fn part2(input: String) {
    let sum = 0;

    println!("Part 2: {:?}", sum);
}