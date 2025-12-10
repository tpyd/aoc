pub fn run(input: &str) -> (u64, u64) {
    let mut tiles: Vec<(i32, i32)> = input
        .trim_end()
        .split("\n")
        .map(|row| {
            let (a, b) = row.split_once(",").unwrap();
            let x = a.parse::<i32>().unwrap();
            let y = b.parse::<i32>().unwrap();
            (x, y)
        })
        .collect();

    let mut max_area = 0;

    for i in 0..tiles.len()-1 {
        for j in i+1..tiles.len() {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];
            let area = ((x2 - x1).abs() + 1) as u64 * ((y2 - y1).abs() + 1) as u64;
            max_area = max_area.max(area);
        }
    }

    let mut edges = Vec::new();
    let mut red_tiles = Vec::new();

    // Get the lines as 'boxes' and corners
    for i in 0..tiles.len() - 1 {
        let first = tiles[i];
        let second = tiles[i + 1];

        let min_x = first.0.min(second.0);
        let min_y = first.1.min(second.1);
        let max_x = first.0.max(second.0);
        let max_y = first.1.max(second.1);

        red_tiles.push(first);
        edges.push((min_x, min_y, max_x, max_y));
    }

    let first = tiles.first().unwrap();
    let last = tiles.last().unwrap();

    let min_x = first.0.min(last.0);
    let min_y = first.1.min(last.1);
    let max_x = first.0.max(last.0);
    let max_y = first.1.max(last.1);

    red_tiles.push(*last);
    edges.push((min_x, min_y, max_x, max_y));

    // Find the largest square
    let mut largest_size = 0;

    for i in 0..tiles.len() - 1 {
        'inner: for j in i + 1..tiles.len() {
            let first = tiles[i];
            let second = tiles[j];

            let min_x = first.0.min(second.0);
            let min_y = first.1.min(second.1);
            let max_x = first.0.max(second.0);
            let max_y = first.1.max(second.1);

            let area = ((max_x - min_x).abs() + 1) as u64 * ((max_y - min_y).abs() + 1) as u64;
            if area > largest_size {
                for edge in &edges {
                    let (edge_min_x, edge_min_y, edge_max_x, edge_max_y) = edge;  
                    if *edge_min_x < max_x && *edge_max_x > min_x && *edge_min_y < max_y && *edge_max_y > min_y {
                        continue 'inner;
                    }
                }

                largest_size = area;
            }
        }
    }

    (max_area, largest_size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_part1() {
        let (part1, _) = run("7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"); 
        assert_eq!(part1, 50);
    }    

    #[test]
    fn test_part2() {
        let (_, part2) = run("7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"); 
        assert_eq!(part2, 24);
    }

    #[test]
    fn test_real() {
        let input = utils::read_input(2025, 9);
        let (part1, part2) = run(&input);

        assert_eq!(part1, 4741451444);
        assert_eq!(part2, 1562459680);
    }
}

