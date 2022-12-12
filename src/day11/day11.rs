pub fn problem1() {
    let contents = include_str!("input.txt");

    let (monkeys, _) = parse_monkeys(contents);
    println!(
        "Day 11, Problem 1: {}",
        get_top_two_product(monkeys, 3, 20, true)
    );
}

pub fn problem2() {
    let contents = include_str!("input.txt");

    let (monkeys, modulo) = parse_monkeys(contents);
    println!(
        "Day 11, Problem 2: {}",
        get_top_two_product(monkeys, modulo, 10000, false)
    );
}

fn get_top_two_product(
    mut monkeys: Vec<Monkey>,
    destresser: i64,
    rounds: usize,
    divide: bool,
) -> i64 {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for item in monkeys[i].items.clone() {
                monkeys[i].inspection_count += 1;
                let new_level = if divide {
                    monkeys[i].operate(item) / destresser
                } else {
                    monkeys[i].operate(item) % destresser
                };
                if new_level % monkeys[i].divisor == 0 {
                    let true_val = monkeys[i].true_val as usize;
                    monkeys[true_val].items.push(new_level);
                } else {
                    let false_val = monkeys[i].false_val as usize;
                    monkeys[false_val].items.push(new_level);
                }
            }
            monkeys[i].items.clear();
        }
    }
    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    monkeys[0].inspection_count * monkeys[1].inspection_count
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(i64),
    Multiply(i64),
    Square,
    None,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    true_val: i64,
    false_val: i64,
    op: Operation,
    divisor: i64,
    inspection_count: i64,
}

impl Monkey {
    fn operate(&self, x: i64) -> i64 {
        // println!("{x}");
        match self.op {
            Operation::Square => x * x,
            Operation::Multiply(y) => x * y,
            Operation::Add(y) => x + y,
            _ => panic!("Shit has gone wrong"),
        }
    }
}

fn parse_monkeys(contents: &str) -> (Vec<Monkey>, i64) {
    let mut monkeys = Vec::new();
    let mut modulo = 1;

    for monkey_lines in contents.split("\n\n") {
        let mut divisor = 0;
        let mut true_val = 0;
        let mut op = Operation::None;
        let mut items = Vec::new();
        for line in monkey_lines.lines() {
            match line.trim().split(" ").collect::<Vec<&str>>().as_slice() {
                ["Starting", "items:", str_items @ ..] => {
                    for item in str_items {
                        items.push(item.replace(",", "").parse::<i64>().unwrap());
                    }
                }
                ["Test:", "divisible", "by", val] => {
                    divisor = val.parse().unwrap();
                    modulo *= divisor;
                }
                ["If", "true:", "throw", "to", "monkey", to] => {
                    true_val = to.parse().unwrap();
                }
                ["If", "false:", "throw", "to", "monkey", to] => {
                    let false_val = to.parse().unwrap();
                    monkeys.push(Monkey {
                        op,
                        true_val,
                        false_val,
                        divisor,
                        items: items.clone(),
                        inspection_count: 0,
                    });
                }
                ["Operation:", "new", "=", "old", str_op, val] => {
                    if *val == "old" {
                        op = Operation::Square;
                    } else if *str_op == "+" {
                        op = Operation::Add(val.parse().unwrap())
                    } else if *str_op == "*" {
                        op = Operation::Multiply(val.parse().unwrap())
                    }
                }
                _ => {}
            }
        }
    }
    (monkeys, modulo)
}
