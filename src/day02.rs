use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    reveals: Vec<Reveal>
}

fn parse_reveal_color_num(text: &str) -> u32 {
    let (num, _) = text.trim().split(' ').collect_tuple().unwrap();
    num.parse::<u32>().unwrap()
}

fn parse_input(input: &str) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();

    for line in input.trim().split('\n') {
        let (gamestr, revealstr) = line.split(':').collect_tuple().unwrap();
        let (_, gamenumstr) = gamestr.split(' ').collect_tuple().unwrap();
        let game_num = gamenumstr.parse::<u32>().unwrap();

        let mut reveals = Vec::new();

        for revealgame in revealstr.split(';') {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for subreveal in revealgame.split(',') {
                if subreveal.contains("red") {
                    red += parse_reveal_color_num(subreveal);
                } else if subreveal.contains("green") {
                    green += parse_reveal_color_num(subreveal); 
                } else if subreveal.contains("blue") {
                    blue += parse_reveal_color_num(subreveal);
                }
            }
            
            let reveal = Reveal{ red: red, green: green, blue: blue };
            reveals.push(reveal);
        }
        
        let game = Game{ id: game_num, reveals: reveals };
        games.push(game);
    }

    games
}

pub fn part1(input: String) {
//     let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let games = parse_input(&input);

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut possible_ids = Vec::new();

    for game in games {
        let mut is_possible = true;
        for reveal in game.reveals {
            if reveal.red > max_red || reveal.green > max_green || reveal.blue > max_blue {
                is_possible = false;
                break;
            }
        }
        if is_possible {
            possible_ids.push(game.id);
        }
    }

    let ans = possible_ids.iter().sum::<u32>();
    println!("{ans}");
}

pub fn part2(input: String) {
//     let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let games = parse_input(&input);

    let mut powers = Vec::new();

    for game in games {
        let maxred = game.reveals.iter().map(|x| x.red).max().unwrap();
        let maxgreen = game.reveals.iter().map(|x| x.green).max().unwrap();
        let maxblue = game.reveals.iter().map(|x| x.blue).max().unwrap();

        powers.push(maxred * maxgreen * maxblue);
    }

    let ans = powers.iter().sum::<u32>();
    println!("{ans}");
}