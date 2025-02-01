pub fn run(input: &str) -> (u32, u32) {
    let mut wrapping_paper = 0;
    let mut ribbon = 0;

    let presents = input.trim().split('\n');

    for present in presents {
        let mut lengths = present.split('x');

        let (l, w, h) = (
            lengths.next().unwrap().parse::<u32>().unwrap(),
            lengths.next().unwrap().parse::<u32>().unwrap(),
            lengths.next().unwrap().parse::<u32>().unwrap(),
        );

        let surface = 2*l*w + 2*w*h + 2*h*l;
        let sides = vec![l*w, w*h, h*l];
        let extra = sides.iter().min().unwrap();

        let total = surface + extra;
        wrapping_paper += total;

        let mut lengths = vec![l, w, h];
        lengths.sort_unstable();
        let side1 = lengths[0];
        let side2 = lengths[1];
        let ribbon_sides = 2*side1 + 2*side2;
        let bow = l * w * h;
        ribbon += ribbon_sides + bow;
    }

    (wrapping_paper, ribbon)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_part1() {
        let (wrapping_paper, _) = run("2x3x4"); 
        assert_eq!(wrapping_paper, 58);

        let (wrapping_paper, _) = run("1x1x10"); 
        assert_eq!(wrapping_paper, 43);
    }    

    #[test]
    fn test_part2() {
        let (_, ribbon) = run("2x3x4");
        assert_eq!(ribbon, 34);

        let (_, ribbon) = run("1x1x10");
        assert_eq!(ribbon, 14);
    }

    #[test]
    fn test_real() {
        let input = utils::get_input(2015, 2);
        let (wrapping_paper, ribbon) = run(&input);

        assert_eq!(wrapping_paper, 1586300);
        assert_eq!(ribbon, 3737498);
    }
}
