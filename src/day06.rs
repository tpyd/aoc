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
        let substr = &input[i..i+num_unique];
        let mut valid = true;

        // Test each character
        for (n1, c1) in substr.chars().enumerate() {
            // Loop again over all characters
            for (n2, c2) in substr.chars().enumerate() {
                // Characters are same and different position
                if c1 == c2 && n1 != n2 {
                    valid = false;
                }
            }
        }

        if valid {
            break
        }

        i += 1;
    }

    let num = i + num_unique;

    println!("Part 2: {:?}", num);
}