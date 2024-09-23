use itertools::Itertools;

// https://doc.rust-lang.org/std/vec/struct.Vec.html#method.windows

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
    const NUM_UNIQUE: usize = 14;

    let mut i = 0;

    loop {
        let substr = &input[i..i+NUM_UNIQUE];
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

    let num = i + NUM_UNIQUE;

    println!("Part 2: {:?}", num);
}