use itertools::Itertools;

struct Monkey {
    number: usize,
    items: Vec<u128>,
    operation: Box<dyn Fn(u128) -> u128>,
    test: Box<dyn Fn(u128) -> bool>,
    true_condition: usize,
    false_condition: usize,
    div_number: u128,
}

impl Monkey {
    fn new(input: &str) -> Self {
        let uinput = input.replace("\r\n", "\n");

        let (num, item_list, op, div, true_test, false_test) = uinput
            .split("\n")
            .collect_tuple()
            .unwrap();

        let number = num
            .split(&[' ', ':'])
            .collect_vec()[1]
            .parse::<usize>().unwrap();

        let itemstr = item_list
            .split(':')
            .collect_vec()[1]
            .trim();

        let items: Vec<u128>;
        if itemstr.is_empty() {
            items = Vec::new();
        } else {
            items = itemstr
                .split(',')
                .map(|x| {
                    str::parse::<u128>(x.trim()).unwrap()
                })
                .collect_vec();
        }

        let (operator, rhs) = op
            .split("new = old ")
            .collect_vec()[1]
            .trim()
            .split(' ')
            .map(|x| x.trim())
            .collect_tuple().unwrap();
        let operation = match rhs {
            "old" => create_operation(rhs, 0),
            _ => {
                let rhs_num = rhs.parse::<u128>().unwrap();
                create_operation(operator, rhs_num)
            }
        };

        let div_number = div
            .split("divisible by ")
            .collect_vec()[1]
            .parse::<u128>().unwrap();
        let test = create_test(div_number);

        let true_condition = true_test
            .split("monkey ")
            .collect_vec()[1]
            .parse::<usize>().unwrap();

        let false_condition = false_test
            .split("monkey ")
            .collect_vec()[1]
            .parse::<usize>().unwrap();

        Monkey {
            number: number,
            items: items,
            operation: operation,
            test: test,
            true_condition: true_condition,
            false_condition: false_condition,
            div_number: div_number,
        }
    }
}

fn create_operation(operator: &str, rhs: u128) -> Box<dyn Fn(u128) -> u128> {
    match operator {
        "*" => Box::new(move |x| x * rhs),
        "+" => Box::new(move |x| x + rhs),
        "old" => Box::new(move |x| x * x),
        _ => panic!(),
    }
}

fn create_test(div_number: u128) -> Box<dyn Fn(u128) -> bool> {
    Box::new(move |value| value % div_number == 0)
}

pub fn part1(input: String) {
    let mut monkeys = input
        .trim()
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|monkey| Monkey::new(monkey))
        .collect::<Vec<Monkey>>();

    let num_monkeys = monkeys.len();

    let mut inspections: Vec<usize> = Vec::new();
    for _ in 0..num_monkeys {
        inspections.push(0);
    }

    for _ in 0..20 {
        for _ in 0..num_monkeys {
            let mut monkey = monkeys.remove(0);

            inspections[monkey.number] += monkey.items.len();

            for _ in 0..monkey.items.len() {
                let mut item = monkey.items.remove(0);
                item = (monkey.operation)(item);
                item = item / 3;

                let give_to_monkey: usize;
                if (monkey.test)(item) {
                    give_to_monkey = monkey.true_condition;
                } else {
                    give_to_monkey = monkey.false_condition;
                }

                for (i, m) in monkeys.iter_mut().enumerate() {
                    if give_to_monkey == m.number {
                        m.items.push(item);
                    }
                }
            }

            monkeys.push(monkey);
        }
    }

    inspections.sort_unstable();
    inspections.reverse();

    let top1 = inspections[0];
    let top2 = inspections[1];

    let monkey_business = top1 * top2;

    println!("Part 1: {:?}", monkey_business);
}

pub fn part2(input: String) {
    let mut monkeys = input
        .trim()
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|monkey| Monkey::new(monkey))
        .collect::<Vec<Monkey>>();

    let num_monkeys = monkeys.len();

    let divisor_product = monkeys.iter().map(|m| m.div_number).product::<u128>();

    let mut inspections: Vec<usize> = Vec::new();
    for _ in 0..num_monkeys {
        inspections.push(0);
    }

    for round in 0..10_000 {
        for _ in 0..num_monkeys {
            let mut monkey = monkeys.remove(0);

            inspections[monkey.number] += monkey.items.len();

            for _ in 0..monkey.items.len() {
                let mut item = monkey.items.remove(0);
                item %= divisor_product;
                item = (monkey.operation)(item);

                let give_to_monkey: usize;
                if (monkey.test)(item) {
                    give_to_monkey = monkey.true_condition;
                } else {
                    give_to_monkey = monkey.false_condition;
                }

                for (i, m) in monkeys.iter_mut().enumerate() {
                    if give_to_monkey == m.number {
                        m.items.push(item);
                    }
                }
            }

            monkeys.push(monkey);
        }
    }

    inspections.sort_unstable();
    inspections.reverse();

    let top1 = inspections[0] as u128;
    let top2 = inspections[1] as u128;

    let monkey_business = top1 * top2;

    println!("Part 2: {:?}", monkey_business);
}
