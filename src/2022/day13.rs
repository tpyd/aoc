use nom::{*, character::{complete::{line_ending, anychar, alpha1, alphanumeric1, newline}, streaming::not_line_ending}, multi::{separated_list1, separated_list0}, bytes::complete::{tag, take_until1}, sequence::{Tuple, delimited}};

#[derive(Debug)]
struct Data(Vec<Packet>);

#[derive(Debug)]
enum Packet {
    Value(u32),
    Data(Vec<Packet>)
}

fn parse_single_packet(input: &str) -> IResult<&str, Packet> {
    let (input, output) = delimited(
        tag("["),
        Parser::map(separated_list0(tag(","), parse_single_packet), Packet::Data),
        tag("]")
    )(input)
        .or_else(|_err| {
            character::complete::u32(input).map(|(input, output)| (input, Packet::Value(output)))
        })?;

    Ok((input, output))
}

fn parse_pairs(input: &str) -> IResult<&str, (Packet, Packet)> {
    let (input, pair1) = take_until1("\n")(input)?;
    let (_, packet1) = parse_single_packet(pair1)?;

    let (input, _) = tag("\n")(input)?;

    let (input, pair2) = take_until1("\n")(input)?;
    let (_, packet2) = parse_single_packet(pair2)?;

    Ok((input, (packet1, packet2)))
}

fn parse_packet_pairs(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    dbg!(&input);
    let (input, pairs) = separated_list1(tag("\n\n"), parse_pairs)(input)?;

    Ok((input, pairs))
}

fn correct_order(p1: &Packet, p2: &Packet) -> bool {
    match (p1, p2) {
        (Packet::Value(v1), Packet::Value(v2)) => v1 == v2,
        _ => false,
    }
}

pub fn part1(input: String) {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    let (_, pairs) = parse_packet_pairs(input).unwrap();

    let mut index_sum = 0;

    for (i, (p1, p2)) in pairs.iter().enumerate() {
        dbg!(p1);
        if correct_order(p1, p2) {
            println!("Equal at index {}", i);
            index_sum += i;
        }
    }

    println!("Part 1: {:?}", index_sum);
}

pub fn part2(input: String) {

    println!("Part 2: {:?}", 0);
}
