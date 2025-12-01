pub fn run(input: &str) -> (i32, i32) {
    let rotations = input
        .trim_end()
        .split('\n')
        .into_iter()
        .map(|x| {
            let (dir, n) = x.split_at(1);
            (dir, n.parse::<i32>().unwrap())
        });

    let mut dial = 50i32;
    let mut zeros_part1 = 0;
    let mut zeros_part2 = 0;

    for (dir, n) in rotations {
        println!("Dial at {}", &dial);
        println!("Rotating {}, {}", &dir, &n);
        match dir {
            "L" => {
                if n >= 100 {
                    zeros_part2 += n / 100;
                }

                let rest = n % 100;
                if rest >= dial && dial != 0 {
                    zeros_part2 += 1;
                }

                if n > dial {
                    dial = (100 - ((n % 100 - dial) % 100)).abs() % 100;
                } else {
                    dial = dial - n;
                }
            },
            "R" => {
                if n >= 100 {
                    zeros_part2 += n / 100;
                }

                let rest = n % 100;
                if rest >= 100 - dial {
                    zeros_part2 += 1;
                }

                // dial = (dial + n) % 100
                dial = (dial + n) % 100;
            },
            _ => {}
        }

        if dial < 0 {
            panic!();
        }

        if dial == 0 {
            zeros_part1 += 1;
        }
        println!("Found {} zero passes", &zeros_part2);
        println!("Dial now at {}", &dial);
        println!("------");
    }
        
    (zeros_part1, zeros_part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_part1() {
//         let (part1, _) = run("L68
// L30
// R48
// L5
// R60
// L55
// L1
// L99
// R14
// L82"); 
//         assert_eq!(part1, 3);

        let (part1, _) = run("L151");
        assert_eq!(part1, 0);

        let (part1, _) = run("L50\nL1\nR1\nR1");
        assert_eq!(part1, 0);
    }    

//     #[test]
//     fn test_part2() {
//         let (_, part2) = run("L68
// L30
// R48
// L5
// R60
// L55
// L1
// L99
// R14
// L82"); 
//         assert_eq!(part2, 6);
//         //
//         let (_, part2) = run("L1000");
//         assert_eq!(part2, 10);
//
//         let (_, part2) = run("L50\nL200");
//         assert_eq!(part2, 3);
//
//         let (_, part2) = run("L75");
//         assert_eq!(part2, 1);
//
//         let (_, part2) = run("L49\nL1\nL99\nL1\nL98\nL2");
//         assert_eq!(part2, 3);
//
//         let (_, part2) = run("L50\nL99");
//         assert_eq!(part2, 1);
//
//         let (_, part2) = run("L49\nL1");
//         assert_eq!(part2, 1);
//
//         let (_, part2) = run("L149");
//         assert_eq!(part2, 1);
//
//         let (_, part2) = run("L150");
//         assert_eq!(part2, 2);
//
//         let (_, part2) = run("L151");
//         assert_eq!(part2, 2);
//
//         let (_, part2) = run("R1000");
//         assert_eq!(part2, 10);
//
//         let (_, part2) = run("R50\nR200");
//         assert_eq!(part2, 3);
//
//         let (_, part2) = run("R75");
//         assert_eq!(part2, 1);
//
//         let (_, part2) = run("R49\nR1\nR99\nR1\nR98\nR2");
//         assert_eq!(part2, 3);
//
//         let (_, part2) = run("R49");
//         assert_eq!(part2, 0);
//
//         let (_, part2) = run("R149");
//         assert_eq!(part2, 1);
//
//         let (_, part2) = run("R150");
//         assert_eq!(part2, 2);
//
//         let (_, part2) = run("R151");
//         assert_eq!(part2, 2);
//     }

    // #[test]
    // fn test_real() {
    //     let input = utils::read_input(2025, 1);
    //     let (part1, part2) = run(&input);
    //
    //     assert_eq!(part1, 1145);
    //     assert_eq!(part2, 6561);
    // }
}

