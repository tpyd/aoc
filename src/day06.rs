use itertools::Itertools;

pub fn part1(input: String) {
    let mut i = 0;

    loop {
        let substr = &input[i..i+4];

        let (a, b, c, d) = substr
            .as_bytes()
            .into_iter()
            .collect_tuple()
            .unwrap();

        if a != b && a != c && a != d && b != c && b != d && c != d {
            break;
        }

        i += 1;
    }

    let num = i + 4;

    println!("Part 1: {:?}", num);
}

pub fn part2(input: String) {
    const num_unique: usize = 14;
    let mut i = 0;

    loop {
        let substr = &input[i..i+4];

        let (a, b, c, d) = substr
            .as_bytes()
            .into_iter()
            .collect_tuple()
            .unwrap();



        i += 1;
    }

    let num = i + 4;

    println!("Part 2: {:?}", num);
}