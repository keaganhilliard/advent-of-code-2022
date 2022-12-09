use std::{collections::HashSet, usize};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

pub fn problem1() {
    let contents = include_str!("input.txt");
    let mut tail_positions: HashSet<Position> = HashSet::new();

    let mut current_head_pos = Position { x: 0, y: 0 };
    let mut current_tail_pos = Position { x: 0, y: 0 };
    tail_positions.insert(current_tail_pos.clone());
    for head_move in contents.split("\n") {
        match head_move.split(" ").collect::<Vec<&str>>().as_slice() {
            ["L", val] => {
                for _ in 0..val.parse::<usize>().unwrap() {
                    current_head_pos = Position {
                        x: current_head_pos.x - 1,
                        y: current_head_pos.y,
                    };
                    current_tail_pos = calculate_new_tail(current_head_pos, current_tail_pos);
                    tail_positions.insert(current_tail_pos);
                }
            }
            ["R", val] => {
                for _ in 0..val.parse::<usize>().unwrap() {
                    current_head_pos = Position {
                        x: current_head_pos.x + 1,
                        y: current_head_pos.y,
                    };
                    current_tail_pos = calculate_new_tail(current_head_pos, current_tail_pos);
                    tail_positions.insert(current_tail_pos);
                }
            }
            ["U", val] => {
                for _ in 0..val.parse::<usize>().unwrap() {
                    current_head_pos = Position {
                        x: current_head_pos.x,
                        y: current_head_pos.y + 1,
                    };
                    current_tail_pos = calculate_new_tail(current_head_pos, current_tail_pos);
                    tail_positions.insert(current_tail_pos);
                }
            }
            ["D", val] => {
                for _ in 0..val.parse::<usize>().unwrap() {
                    current_head_pos = Position {
                        x: current_head_pos.x,
                        y: current_head_pos.y - 1,
                    };
                    current_tail_pos = calculate_new_tail(current_head_pos, current_tail_pos);
                    tail_positions.insert(current_tail_pos);
                }
            }
            _ => {}
        }
    }
    println!("Day 9, Problem 1: {:?}", tail_positions.len());
}

fn calculate_new_tail(head: Position, tail: Position) -> Position {
    let mut new_tail = tail;
    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;
    match x_diff {
        x if x > 1 => {
            new_tail = Position {
                x: new_tail.x + 1,
                y: new_tail.y,
            };
            match y_diff {
                y if y > 0 => {
                    return Position {
                        x: new_tail.x,
                        y: new_tail.y + 1,
                    };
                }
                y if y < 0 => {
                    return Position {
                        x: new_tail.x,
                        y: new_tail.y - 1,
                    };
                }
                _ => {}
            }
        }
        x if x < -1 => {
            new_tail = Position {
                x: new_tail.x - 1,
                y: new_tail.y,
            };
            match y_diff {
                y if y > 0 => {
                    return Position {
                        x: new_tail.x,
                        y: new_tail.y + 1,
                    };
                }
                y if y < 0 => {
                    return Position {
                        x: new_tail.x,
                        y: new_tail.y - 1,
                    };
                }
                _ => {}
            }
        }
        _ => {}
    }
    match y_diff {
        y if y > 1 => {
            new_tail = Position {
                x: new_tail.x,
                y: new_tail.y + 1,
            };
            match x_diff {
                x if x > 0 => {
                    return Position {
                        x: new_tail.x + 1,
                        y: new_tail.y,
                    };
                }
                x if x < 0 => {
                    return Position {
                        x: new_tail.x - 1,
                        y: new_tail.y,
                    };
                }
                _ => {}
            }
        }
        y if y < -1 => {
            new_tail = Position {
                x: new_tail.x,
                y: new_tail.y - 1,
            };
            match x_diff {
                diff if diff > 0 => {
                    return Position {
                        x: new_tail.x + 1,
                        y: new_tail.y,
                    };
                }
                diff if diff < 0 => {
                    return Position {
                        x: new_tail.x - 1,
                        y: new_tail.y,
                    };
                }
                _ => {}
            }
        }
        _ => {}
    }
    new_tail
}
pub fn problem2() {
    let contents = include_str!("input.txt");
    let mut tail_positions: HashSet<Position> = HashSet::new();

    let mut current_head_pos = Position { x: 0, y: 0 };
    let mut current_tail_pos = Position { x: 0, y: 0 };
    let mut knots = vec![
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 0 },
    ];
    tail_positions.insert(current_tail_pos.clone());

    for head_move in contents.split("\n") {
        match head_move.split(" ").collect::<Vec<&str>>().as_slice() {
            ["L", val] => {
                for _ in 0..val.parse::<usize>().unwrap() {
                    current_head_pos = Position {
                        x: current_head_pos.x - 1,
                        y: current_head_pos.y,
                    };

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
            ["R", val] => {
                for _ in 0..val.parse::<usize>().unwrap() {
                    current_head_pos = Position {
                        x: current_head_pos.x + 1,
                        y: current_head_pos.y,
                    };
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
            ["U", val] => {
                for _ in 0..val.parse::<usize>().unwrap() {
                    current_head_pos = Position {
                        x: current_head_pos.x,
                        y: current_head_pos.y + 1,
                    };
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
            ["D", val] => {
                for _ in 0..val.parse::<usize>().unwrap() {
                    current_head_pos = Position {
                        x: current_head_pos.x,
                        y: current_head_pos.y - 1,
                    };
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
