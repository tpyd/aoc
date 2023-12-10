use itertools::Itertools;

fn has_symbol(c: &char) -> bool {
    !c.is_digit(10) && *c != '.'
}

pub fn part1(input: String) {
//     let input = "467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..";

    let mut lines = Vec::new();
    for line in input.trim().split('\n') {
        // Pad start and end of line
        let newline = ".".to_owned() + line + ".";
        lines.push(newline); 
    }

    // Pad empty line above and after
    let linelength = lines[0].len();
    let pad = ".".repeat(linelength);
    lines.insert(0, pad.clone());
    lines.push(pad);

    let mut numbers = Vec::new();

    for line_index in 1..lines.len() - 1 {
        let line = &lines[line_index];
        let mut digits_to_skip = 0;

        for (char_index, c) in line.char_indices() {
            if digits_to_skip > 0 {
                digits_to_skip -= 1;
                continue;
            }

            if c.is_digit(10) {
                let numberstart = &line[char_index..];
                let number_str = numberstart.chars().by_ref().take_while(|c: &char| c.is_digit(10)).collect::<String>();
                digits_to_skip = number_str.len();
                numbers.push((number_str, line_index, char_index));
            }
        }
    }

    let mut sum = 0;

    for (num, row, col) in numbers {
        let num_length = num.len();

        let upper_row = &lines[row-1];
        let upper_chars = &upper_row[col-1..col+num_length+1];

        let bottom_row = &lines[row+1];
        let bottom_chars = &bottom_row[col-1..col+num_length+1];

        let current_row = &lines[row];
        let front = &current_row.chars().nth(col-1).unwrap();
        let back = &current_row.chars().nth(col+num_length).unwrap();

        let mut is_part = false;
            
        // Check everything
        for c in upper_chars.chars() {
            if has_symbol(&c) {
                is_part = true;
            }
        }

        for c in bottom_chars.chars() {
            if has_symbol(&c) {
                is_part = true;
            }
        }

        if has_symbol(front) {
            is_part = true;
        }

        if has_symbol(back) {
            is_part = true;
        }

        if is_part {
            let ans = num.parse::<u32>().unwrap();
            sum += ans;
        }
    }

    println!("{sum}");
}

pub fn part2(input: String) {
    // println!("{ans}");
}