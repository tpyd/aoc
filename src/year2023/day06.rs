use itertools::Itertools;

fn part1(input: String) {
    let (times, records) = input.split('\n').collect_tuple().unwrap();
    let mut times = times.split_whitespace().collect_vec();
    let mut records = records.split_ascii_whitespace().collect_vec();

    times.remove(0);
    records.remove(0);

    let times = times.iter().map(|x| x.parse::<u32>().unwrap()).collect_vec();
    let records = records.iter().map(|x| x.parse::<u32>().unwrap()).collect_vec();

    let mut unique = 1;

    for (time, record) in times.iter().zip(records.iter()) {
        let mut unique_ways = 0;
        for speed in 0..=*time {
            let distance = (time - speed) * speed;
            if distance > *record {
                unique_ways += 1;
            }
        }
        unique *= unique_ways;
    }

    println!("{unique}");
}

fn part2(input: String) {
    let (times, records) = input
        .split('\n')
        .collect_tuple()
        .unwrap();
    
    let times: String = times.split(':').skip(1).collect();
    let time = times
        .replace(" ", "")
        .parse::<u64>().unwrap(); 

    let records: String = records.split(':').skip(1).collect();
    let record = records
        .replace(" ", "")
        .parse::<u64>().unwrap();

    let mut first_win = 0;
    let mut last_win = 0;

    for speed in 0..=time {
        let distance = (time - speed) * speed;
        if distance > record {
            first_win = speed;
            break;
        }
    }

    for speed in (0..=time).rev() {
        let distance = (time - speed) * speed;
        if distance > record {
            last_win = speed;
            break;
        }
    }

    let ans = last_win - first_win + 1;

    println!("{ans}");
}

///
/// Can use the quadratic formula to solve this problem. 
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
fn solve(time: f32, record: f32) -> u32 {
    let root = f32::sqrt(time * time - 4. * record);
    let lower = (((time - root) / 2.) + 1.).floor() as u32;
    let upper = ((time + root) / 2.).ceil() as u32;

    (lower..upper).len() as u32
}

pub fn run(input: String) {
    part1(input.clone());
    part2(input.clone());
}
