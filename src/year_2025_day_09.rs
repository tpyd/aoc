pub fn run(input: &str) -> (i64, i64) {
    let tiles_str: Vec<&str> = input
        .trim_end()
        .split("\n")
        .collect();

    let mut edges = Vec::new();
    let mut tiles = Vec::new();

    for i in 0..tiles_str.len() - 1 {
        let (x1_str, y1_str) = tiles_str[i].split_once(',').unwrap();
        let (x2_str, y2_str) = tiles_str[i + 1].split_once(',').unwrap();

        let x1 = x1_str.parse::<i64>().unwrap();
        let y1 = y1_str.parse::<i64>().unwrap();
        let x2 = x2_str.parse::<i64>().unwrap();
        let y2 = y2_str.parse::<i64>().unwrap();

        let min_x = x1.min(x2);
        let min_y = y1.min(y2);
        let max_x = x1.max(x2);
        let max_y = y1.max(y2);

        tiles.push((x1, y1));
        edges.push((min_x, min_y, max_x, max_y));
    }

    let (x1_str, y1_str) = tiles_str[tiles_str.len() - 1].split_once(',').unwrap();
    let (x2_str, y2_str) = tiles_str[0].split_once(',').unwrap();

    let x1 = x1_str.parse::<i64>().unwrap();
    let y1 = y1_str.parse::<i64>().unwrap();
    let x2 = x2_str.parse::<i64>().unwrap();
    let y2 = y2_str.parse::<i64>().unwrap();

    let min_x = x1.min(x2);
    let min_y = y1.min(y2);
    let max_x = x1.max(x2);
    let max_y = y1.max(y2);

    tiles.push((x1, y1));
    edges.push((min_x, min_y, max_x, max_y));

    let mut largest_part1 = 0;
    let mut largest_part2 = 0;

    for i in 0..tiles.len() - 1 {
        'inner: for j in i + 1..tiles.len() {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];

            let min_x = x1.min(x2);
            let min_y = y1.min(y2);
            let max_x = x1.max(x2);
            let max_y = y1.max(y2);

            let area = (max_x - min_x + 1) * (max_y - min_y + 1);
            
            if area > largest_part1 {
                largest_part1 = area;
            }

            if area > largest_part2 {
                for edge in &edges {
                    let (e_min_x, e_min_y, e_max_x, e_max_y) = edge;  
                    if *e_min_x < max_x && *e_max_x > min_x && *e_min_y < max_y && *e_max_y > min_y {
                        continue 'inner;
                    }
                }

                largest_part2 = area;
            }
        }
    }

    (largest_part1, largest_part2)
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

