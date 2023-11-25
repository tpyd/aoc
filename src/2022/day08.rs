// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust

fn is_visible(forest: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let forest_len = forest.len();

    // From top
    let mut tallest = 0;
    for i in 0..y {
        let tree = forest[i][x];
        if tree > tallest {
            tallest = tree;
        }
    }
    if tallest < forest[y][x] {
        return true
    }

    // From left
    let mut tallest = 0;
    for i in 0..x {
        let tree = forest[y][i];
        if tree > tallest {
            tallest = tree;
        }
    }
    if tallest < forest[y][x] {
        return true
    }

    // From right
    let mut tallest = 0;
    for i in (x+1..forest_len).rev() {
        let tree = forest[y][i];
        if tree > tallest {
            tallest = tree;
        }
    }
    if tallest < forest[y][x] {
        return true
    }

    // From bottom
    let mut tallest = 0;
    for i in (y+1..forest_len).rev() {
        let tree = forest[i][x];
        if tree > tallest {
            tallest = tree;
        }
    }
    if tallest < forest[y][x] {
        return true
    }

    false
}

fn get_scenic_score(forest: &Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
    let house = forest[y][x];
    let forest_len = forest.len();

    // To top
    let mut top_score = 0;
    for i in (0..y).rev() {
        let tree = forest[i][x];
        if tree < house {
            top_score += 1;
        } else {
            top_score += 1;
            break
        }
    }

    // To left
    let mut left_score = 0;
    for i in (0..x).rev() {
        let tree = forest[y][i];
        if tree < house {
            left_score += 1;
        } else {
            left_score += 1;
            break
        }
    }

    // To right
    let mut right_score = 0;
    for i in x+1..forest_len {
        let tree = forest[y][i];
        if tree < house {
            right_score += 1;
        } else {
            right_score += 1;
            break
        }
    }

    // To bottom
    let mut bottom_score = 0;
    for i in y+1..forest_len {
        let tree = forest[i][x];
        if tree < house {
            bottom_score += 1;
        } else {
            bottom_score += 1;
            break
        }
    }

    let scenic_score = top_score * left_score * right_score * bottom_score;
    scenic_score
}

pub fn part1(input: String) {
    let forest: Vec<Vec<u8>> = input
        .trim()
        .split('\n')
        .map(|x| {
            x
                .as_bytes()
                .into_iter()
                .map(|x| x - 48) // Fix this, should be a direct way to parse
                .collect()
        })
        .to_owned()
        .collect();

    let mut visible = 0;

    // Go through inner forest
    let forest_len = forest.len();
    for y in 1..forest_len-1 {
        for x in 1..forest_len-1 {
            if is_visible(&forest, x, y) {
                visible += 1;
            }
        }
    }

    // Add edges
    let outer_visible = (forest_len - 1) * 4;
    visible += outer_visible;

    println!("Part 1: {:?}", visible);
}

pub fn part2(input: String) {
    let forest: Vec<Vec<u8>> = input
        .trim()
        .split('\n')
        .map(|x| {
            x
                .as_bytes()
                .into_iter()
                .map(|x| x - 48) // Fix this, should be a direct way to parse
                .collect()
        })
        .to_owned()
        .collect();

    let mut scenic_scores: Vec<u32> = Vec::new();

    // Go through inner forest
    let forest_len = forest.len();
    for y in 1..forest_len-1 {
        for x in 1..forest_len-1 {
            let scenic_score = get_scenic_score(&forest, x, y);
            scenic_scores.push(scenic_score);
        }
    }

    let best_score = scenic_scores.iter().max().unwrap();

    println!("Part 2: {:?}", best_score);
}
