use itertools::Itertools;

/// Gives the number of ways one can beat the record.
/// 
/// Solves the problem using the quadratic formula. 
/// Let t be the time you have, r the record for the race
/// and h the hold time for the button.
/// 
/// Need to solve for h: 
///         (t - h) h > r
///          th - h^2 > r
///     -h^2 + th - r > 0
/// quadratic formula:
///     h = -t +- sqrt(t^2 - 4 * (-1) * (-r)) / 2 * (-1)
///       = -t +- sqrt(t^2 - 4r) / -2
///       = t -+ sqrt(t^2 - 4r) / 2 
///
fn solve(time: f64, record: f64) -> u32 {
    let root = f64::sqrt(time * time - 4. * record);
    let lower = (((time - root) / 2.) + 1.).floor() as u32;
    let upper = ((time + root) / 2.).ceil() as u32;

    (lower..upper).len() as u32
}

fn part1(input: String) {
    let (times, records) = input
        .split('\n')
        .map(|line| { 
            line
                .split_whitespace()
                .skip(1)
                .map(|x| x.parse::<f64>().unwrap())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();

    let ans: u32 = times
        .iter()
        .zip(records.iter())
        .map(|(t, r)| solve(*t, *r))
        .product();

    println!("{ans}");
}

fn part2(input: String) {
    let (time, record) = input
        .split('\n')
        .map(|line| {
            line
                .split(':')
                .skip(1)
                .collect::<String>()
                .replace(" ", "")
                .parse::<f64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    let ans = solve(time, record);

    println!("{ans}");
}

pub fn run(input: String) {
    part1(input.clone());
    part2(input.clone());
}
