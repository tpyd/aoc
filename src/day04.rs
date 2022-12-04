use itertools::Itertools;

pub fn part1(input: String) {
    let sum = input
        .trim()
        .split('\n')
        .map(|pair| {
            let assignments = pair
                .split(',')
                .map(|assignment| {
                    assignment
                        .split('-')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect_tuple::<(u32, u32)>().unwrap()
                })
                .collect_tuple::<((u32, u32), (u32, u32))>().unwrap();

            let ((a1, a2), (b1, b2)) = assignments;

            // a contains b
            if a1 <= b1 && a2 >= b2 {
                return true
            }

            // b contains a
            if b1 <= a1 && b2 >= a2 {
                return true
            }

            false
        })
        .filter(|x| *x)
        .count();

    println!("Number of assignments where one fully contains the other: {:?}", sum);
}

pub fn part2(input: String) {
    let sum = input
        .trim()
        .split('\n')
        .map(|pair| {
            let assignments = pair
                .split(',')
                .map(|assignment| {
                    assignment
                        .split('-')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect_tuple::<(u32, u32)>().unwrap()
                })
                .collect_tuple::<((u32, u32), (u32, u32))>().unwrap();

            let ((a1, a2), (b1, b2)) = assignments;

            if a1 > b2 || b1 > a2 {
                return false
            }

            true
        })
        .filter(|x| *x)
        .count();

    println!("Number of assignments overlaps {:?}", sum);
}