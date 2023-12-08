use itertools::Itertools;
use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::{tuple, terminated}, character::complete::{char, digit1}, multi::separated_list1, branch::alt};

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue
}

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

impl Reveal {
    fn new() -> Self {
        Reveal { red: 0, green: 0, blue: 0 }
    }
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, color) = alt((tag("red"), tag("green"), tag("blue")))(input)?;
    match color {
        "red" => Ok(("", Color::Red)),
        "green" => Ok(("", Color::Green)),
        "blue" => Ok(("", Color::Blue)),
        _ => panic!()
    }
}

fn parse_num(input: &str) -> IResult<&str, (u32, Color)> {
    let (input, number) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = parse_color(input)?;

    Ok((input, (number, color)))
}

fn parse_reveal(input: &str) -> IResult<&str, Reveal> {
    let (input, color_reveals) = separated_list1(tag(", "), parse_num)(input)?;

    let mut reveal = Reveal::new();
    for (num, color) in color_reveals {
        match color {
            Color::Red => reveal.red += num,
            Color::Green => reveal.green += num,
            Color::Blue => reveal.blue += num,
        }
    }

    Ok((input, reveal))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, game_id) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, reveals) = separated_list1(tag("; "), parse_reveal)(input)?;

    Ok(("", Game{ id: game_id, reveals: reveals }))
}


fn parse_reveal_color_num(text: &str) -> u32 {
    let (num, _) = text.trim().split(' ').collect_tuple().unwrap();
    num.parse::<u32>().unwrap()
}


pub fn part1(input: String) {
//     let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let mut games: Vec<Game> = Vec::new();

    for line in input.trim().split('\n') {
        // let game = parse_game(input).unwrap().1;
        // games.push(game);

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

}