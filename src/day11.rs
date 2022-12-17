use itertools::Itertools;

struct Monkey {
    number: i32,
    items: Vec<i32>,
    operation: Box<dyn Fn(i32) -> i32>,
    test: Box<dyn Fn(i32) -> bool>,
    true_condition: i32,
    false_condition: i32,
}

impl Monkey {
    fn new(input: &str) -> Self {
        let (num, item_list, op, div, true_test, false_test) = input
            .split('\n')
            .collect_tuple()
            .unwrap();

        let number = num
            .split(&[' ', ':'])
            .collect_vec()[1]
            .parse::<i32>().unwrap();

        let items = item_list
            .split(':')
            .collect_vec()[1]
            .trim()
            .split(' ')
            .map(|x| {
                str::parse::<i32>(x.trim()).unwrap()
            })
            .collect_vec();

        let (operator, rhs) = op
            .split("new = old ")
            .collect_vec()[1]
            .split(' ')
            .collect_tuple().unwrap();
        let rhs_num = rhs.parse::<i32>().unwrap();
        let operation = create_operation(operator, rhs_num);

        let div_number = div
            .split("divisible by ")
            .collect_vec()[1]
            .parse::<i32>().unwrap();
        let test = create_test(div_number);

        let true_condition = true_test
            .split("monkey ")
            .collect_vec()[1]
            .parse::<i32>().unwrap();

        let false_condition = false_test
            .split("monkey ")
            .collect_vec()[1]
            .parse::<i32>().unwrap();

        Monkey {
            number: number,
            items: items,
            operation: operation,
            test: test,
            true_condition: true_condition,
            false_condition: false_condition,
        }
    }
}

fn create_operation(operator: &str, rhs: i32) -> Box<dyn Fn(i32) -> i32> {
    match operator {
        "*" => Box::new(move |x| x * rhs),
        "+" => Box::new(move |x| x + rhs),
        _ => panic!(),
    }
}

fn create_test(div_number: i32) -> Box<dyn Fn(i32) -> bool> {
    Box::new(move |value| value % div_number == 0)
}

pub fn part1(input: String) {
    let mut monkeys = input
        .trim()
        .split('\n')
        .map(|monkey| Monkey::new(monkey))
        .collect::<Vec<Monkey>>();

    let num_monkeys = monkeys.len();

    for _ in 0..20 {
        for _ in 0..num_monkeys {
            let mut monkey = monkeys.remove(0);
            for _ in 0..monkey.items.len() {
                let item = monkey.items.remove(0);
                let new_item = (monkey.operation)(item) / 3;
                if (monkey.test)(new_item) {
                    for (i, m) in monkeys.into_iter().enumerate() {
                        if m.number == monkey.true_condition {
                            monkeys[i].items.push(new_item);
                        }
                    }
                    monkeys[monkey.true_condition as usize].items.push(new_item);
                } else {
                    for (i, m) in monkeys.into_iter().enumerate() {
                        if m.number == monkey.true_condition {
                            monkeys[i].items.push(new_item);
                        }
                    }
                    monkeys[monkey.false_condition as usize].items.push(new_item);
                }
            }

            monkeys.push(monkey);
        }
    }

    println!("Part 1: {:?}", 0);
}

pub fn part2(input: String) {

    println!("Part 2: {:?}", 0);
}
