use itertools::Itertools;

pub fn part1(input: String) {
    let sum = input
        .trim()
        .split('\n')
        .map(|pair| {
            let assignments: (u32, u32, u32, u32) = pair
                .split(|x| !char::is_numeric(x))
                .map(|x| x.parse::<u32>().unwrap())
                .collect_tuple().unwrap();

            let (a1, a2, b1, b2) = assignments;

            if (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2) {
                return true
            }

            false
        })
        .filter(|x| *x)
        .count();

    println!("Part 1: {:?}", sum);
}

pub fn part2(input: String) {
    let sum = input
        .trim()
        .split('\n')
        .map(|pair| {
            let assignments: (u32, u32, u32, u32) = pair
                .split(|x| !char::is_numeric(x))
                .map(|x| x.parse::<u32>().unwrap())
                .collect_tuple().unwrap();

            let (a1, a2, b1, b2) = assignments;

            if a1 > b2 || b1 > a2 {
                return false
            }

            true
        })
        .filter(|x| *x)
        .count();

    println!("Part 2: {:?}", sum);
}
