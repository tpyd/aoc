use itertools::Itertools;

#[derive(Debug)]
struct Card {
    win_nums: Vec<u32>,
    my_nums: Vec<u32>
}

fn parse_list(input: &str) -> Vec<u32> {
    input
        .trim()
        .replace("  ", " ")
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect_vec()
}

pub fn part1(input: String) {
//     let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let mut cards = Vec::new();

    for card_str in input.trim().split('\n') {
        let (_, card_input) = card_str.split(':').collect_tuple().unwrap();
        let (win_str, my_str) = card_input.split('|').collect_tuple().unwrap();

        let win_nums = parse_list(win_str);
        let my_nums = parse_list(my_str);

        let card = Card{ win_nums: win_nums, my_nums: my_nums };
        cards.push(card);
    }

    let mut sum: u32 = 0;

    for card in cards {
        let mut matches = 0;

        for num in card.my_nums {
            if card.win_nums.contains(&num) {
                matches += 1;
            }
        }

        if matches > 0 {
            sum += 2u32.pow(matches - 1); 
        }
    }

    println!("{sum}");
}

pub fn part2(input: String) {
    // println!("{sum}");
}
