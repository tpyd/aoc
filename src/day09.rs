use std::collections::HashSet;

use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn distance(a: Point, b: Point) -> (u32, u32) {
    (a.x.abs_diff(b.x), a.y.abs_diff(b.y))
}

pub fn part1(input: String) {
    let instructions = input
        .trim()
        .split('\n')
        .map(|x| {
            let (d, n) = x
                .split(' ')
                .collect_tuple().unwrap();

            let m = str::parse::<u32>(n).unwrap();
            (d, m)
        })
        .collect::<Vec<(&str, u32)>>();

    let mut head = Point{x: 0, y: 0};
    let mut tail = Point{x: 0, y: 0};

    let mut tail_pos: HashSet<Point> = HashSet::new();

    for (dir, n) in instructions {
        for _ in 0..n {
            match dir {
                "R" => head.x += 1,
                "L" => head.x -= 1,
                "U" => head.y += 1,
                "D" => head.y -= 1,
                _ => panic!(),
            }

            // Update tail
            let (xdiff, ydiff) = distance(head, tail);

            // Move tail
            if xdiff + ydiff == 3 || xdiff == 2 || ydiff == 2  {
                tail.x += (head.x - tail.x).clamp(-1, 1);
                tail.y += (head.y - tail.y).clamp(-1, 1);
            }

            tail_pos.insert(tail);
        }
    }

    let num_tail_pos = tail_pos.len();

    println!("Part 1: {:?}", num_tail_pos);
}

pub fn part2(input: String) {

    println!("Part 2: {:?}", 0);
}
