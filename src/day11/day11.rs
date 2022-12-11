use std::collections::HashMap;

pub fn problem1() {
    let contents = include_str!("input.txt");

    let (mut monkeys, _) = parse_monkeys(contents);
    let mut throw_to: HashMap<i64, Vec<i64>> = HashMap::new();
    for _ in 0..20 {
        for monkey in &mut monkeys {
            if throw_to.contains_key(&monkey.index) {
                monkey
                    .items
                    .append(throw_to.get_mut(&monkey.index).unwrap());
                throw_to.remove(&monkey.index);
            }
            for item in &monkey.items {
                monkey.inspection_count += 1;
                let new_level = monkey.operate(*item) / 3;
                if new_level % monkey.divisor == 0 {
                    if throw_to.contains_key(&monkey.true_val) {
                        let throw = throw_to.get_mut(&monkey.true_val).unwrap();
                        throw.push(new_level)
                    } else {
                        throw_to.insert(monkey.true_val, vec![new_level]);
                    }
                } else {
                    if throw_to.contains_key(&monkey.false_val) {
                        let throw = throw_to.get_mut(&monkey.false_val).unwrap();
                        throw.push(new_level)
                    } else {
                        throw_to.insert(monkey.false_val, vec![new_level]);
                    }
                }
            }
            monkey.items.clear();
        }
        for monkey in &mut monkeys {
            if throw_to.contains_key(&monkey.index) {
                monkey
                    .items
                    .append(throw_to.get_mut(&monkey.index).unwrap());
                throw_to.remove(&monkey.index);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    println!(
        "Day 11, Problem 1: {}",
        monkeys[0].inspection_count * monkeys[1].inspection_count
    );
}

pub fn problem2() {
    let contents = include_str!("input.txt");

    let (mut monkeys, modulo) = parse_monkeys(contents);
    let mut iterable_monkeys = monkeys.clone();
    for _ in 0..10000 {
        for i in 0..iterable_monkeys.len() {
            let current_monkey = &iterable_monkeys[i];
            for item in &current_monkey.items {
                (&mut monkeys[i]).inspection_count += 1;
                let new_level = current_monkey.operate(*item);
                if new_level % current_monkey.divisor == 0 {
                    (&mut monkeys[current_monkey.true_val as usize])
                        .items
                        .push(new_level % modulo);
                } else {
                    (&mut monkeys[current_monkey.false_val as usize])
                        .items
                        .push(new_level % modulo);
                }
            }
            (&mut monkeys[i]).items.clear();
            iterable_monkeys = monkeys.clone()
        }
    }

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    println!(
        "Day 11, Problem 2: {}",
        monkeys[0].inspection_count * monkeys[1].inspection_count
    );
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
    index: i64,
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
    let mut monkey_index = 0;
    let mut divisor = 0;
    let mut true_val = 0;
    let mut monkey_op = Operation::None;
    let mut monkey_items = Vec::new();
    let mut monkeys = Vec::new();
    let mut modulo = 1;

    for line in contents.lines() {
        match line.trim().split(" ").collect::<Vec<&str>>().as_slice() {
            ["Monkey", str_index] => {
                monkey_index = str_index.replace(":", "").parse::<i64>().unwrap();
            }
            ["Starting", "items:", items @ ..] => {
                for item in items {
                    monkey_items.push(item.replace(",", "").parse::<i64>().unwrap());
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
                monkeys.push(Monkey {
                    index: monkey_index,
                    items: monkey_items,
                    op: monkey_op.clone(),
                    true_val,
                    false_val: to.parse().unwrap(),
                    divisor,
                    inspection_count: 0,
                });
                monkey_items = Vec::new()
            }
            ["Operation:", "new", "=", "old", op, val] => {
                if *val == "old" {
                    monkey_op = Operation::Square;
                } else if *op == "+" {
                    monkey_op = Operation::Add(val.parse().unwrap())
                } else if *op == "*" {
                    monkey_op = Operation::Multiply(val.parse().unwrap())
                }
            }
            _ => {}
        }
    }
    (monkeys, modulo)
}
