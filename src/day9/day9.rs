use std::{collections::HashSet, usize};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }
}

pub fn problem1() {
    let contents = include_str!("input.txt");
    let mut tail_positions: HashSet<Position> = HashSet::new();

    let mut current_head_pos = Position::new();
    let mut current_tail_pos = Position::new();
    tail_positions.insert(current_tail_pos.clone());
    for head_move in contents.split("\n") {
        match head_move.split(" ").collect::<Vec<&str>>().as_slice() {
            [dir, val] => {
                for _ in 0..val.parse::<usize>().unwrap() {
                    current_head_pos = calculate_new_head(*dir, current_head_pos);
                    current_tail_pos = calculate_new_tail(current_head_pos, current_tail_pos);
                    tail_positions.insert(current_tail_pos);
                }
            }
            _ => {}
        }
    }
    println!("Day 9, Problem 1: {:?}", tail_positions.len());
}

pub fn problem2() {
    let contents = include_str!("input.txt");
    let mut tail_positions: HashSet<Position> = HashSet::new();

    let mut current_head_pos = Position::new();
    let mut current_tail_pos = Position::new();
    let mut knots = Vec::new();
    for _ in 0..8 {
        knots.push(Position::new())
    }
    tail_positions.insert(current_tail_pos.clone());

    for head_move in contents.split("\n") {
        match head_move.split(" ").collect::<Vec<&str>>().as_slice() {
            [dir, val] => {
                for _ in 0..val.parse::<usize>().unwrap() {
                    current_head_pos = calculate_new_head(*dir, current_head_pos);
                    let mut previous_pos = current_head_pos;
                    for knot in &mut knots {
                        previous_pos = calculate_new_tail(previous_pos, knot.clone());
                        knot.x = previous_pos.x;
                        knot.y = previous_pos.y;
                    }
                    current_tail_pos = calculate_new_tail(previous_pos, current_tail_pos);
                    tail_positions.insert(current_tail_pos);
                }
            }
            _ => {}
        }
    }
    println!("Day 9, Problem 2: {:?}", tail_positions.len());
}

fn calculate_new_head(dir: &str, current_head_pos: Position) -> Position {
    let (x, y) = match dir {
        "L" => (current_head_pos.x - 1, current_head_pos.y),
        "R" => (current_head_pos.x + 1, current_head_pos.y),
        "U" => (current_head_pos.x, current_head_pos.y + 1),
        "D" => (current_head_pos.x, current_head_pos.y - 1),
        _ => (current_head_pos.x, current_head_pos.y),
    };
    Position { x, y }
}

fn calculate_new_tail(head: Position, tail: Position) -> Position {
    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;

    if x_diff.abs() > 1 {
        Position {
            x: tail.x + get_adder(x_diff, 1),
            y: tail.y + get_adder(y_diff, 0),
        }
    } else if y_diff.abs() > 1 {
        Position {
            x: tail.x + get_adder(x_diff, 0),
            y: tail.y + get_adder(y_diff, 1),
        }
    } else {
        tail
    }
}

fn get_adder(diff: i32, limit: i32) -> i32 {
    if diff > limit {
        1
    } else if diff < limit {
        -1
    } else {
        0
    }
}
