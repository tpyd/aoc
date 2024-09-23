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

    let num_ropes = 10;
    let mut ropes: Vec<Point> = Vec::new();
    for _ in 0..num_ropes {
        ropes.push(Point{x: 0, y: 0});
    }

    let mut tail_pos: HashSet<Point> = HashSet::new();

    for (dir, n) in instructions {
        for _ in 0..n {
            let mut head = &mut ropes[0];
            match dir {
                "R" => head.x += 1,
                "L" => head.x -= 1,
                "U" => head.y += 1,
                "D" => head.y -= 1,
                _ => panic!(),
            }

            // Update ropes
            for i in 0..num_ropes-1 {
                let a = ropes[i];
                let mut b = &mut ropes[i+1];

                let (xdiff, ydiff) = distance(a, *b);

                // Move rope
                if xdiff + ydiff == 3 || xdiff == 2 || ydiff == 2  {
                    b.x += (a.x - b.x).clamp(-1, 1);
                    b.y += (a.y - b.y).clamp(-1, 1);
                }
            }

            tail_pos.insert(*ropes.last().unwrap());
        }
    }

    let num_tail_pos = tail_pos.len();

    println!("Part 2: {:?}", num_tail_pos);
}
