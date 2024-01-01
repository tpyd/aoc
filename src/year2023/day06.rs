use itertools::Itertools;

pub fn run(input: String) {
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
