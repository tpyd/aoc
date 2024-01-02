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

    // Off by one error somewhere i think
    let ans = last_win - first_win + 1;

    println!("{ans}");
}

pub fn run(input: String) {
    part1(input.clone());
    part2(input.clone());
}
