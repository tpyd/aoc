pub fn run(input: &str) -> (u64, i32) {
    let tiles: Vec<(i32, i32)> = input
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

    (max_area, 0)
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

    // #[test]
    // fn test_part2() {
    //     let (_, part2) = run("example"); 
    //     assert_eq!(part2, );
    // }
    //
    // #[test]
    // fn test_real() {
    //     let input = utils::read_input(2025, 9);
    //     let (part1, part2) = run(&input);
    //
    //     assert_eq!(part1, 4741451444);
    //     assert_eq!(part2, );
    // }
}

