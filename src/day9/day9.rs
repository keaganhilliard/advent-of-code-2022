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
            [dir, val] => {
                for _ in 0..val.parse::<usize>().unwrap() {
                    current_head_pos = match *dir {
                        "L" => Position {
                            x: current_head_pos.x - 1,
                            y: current_head_pos.y,
                        },
                        "R" => Position {
                            x: current_head_pos.x + 1,
                            y: current_head_pos.y,
                        },
                        "U" => Position {
                            x: current_head_pos.x,
                            y: current_head_pos.y + 1,
                        },
                        "D" => Position {
                            x: current_head_pos.x,
                            y: current_head_pos.y - 1,
                        },
                        _ => current_head_pos,
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

pub fn problem2() {
    let contents = include_str!("input.txt");
    let mut tail_positions: HashSet<Position> = HashSet::new();

    let mut current_head_pos = Position { x: 0, y: 0 };
    let mut current_tail_pos = Position { x: 0, y: 0 };
    let mut knots = Vec::new();
    for _ in 0..8 {
        knots.push(Position { x: 0, y: 0 })
    }
    tail_positions.insert(current_tail_pos.clone());

    for head_move in contents.split("\n") {
        match head_move.split(" ").collect::<Vec<&str>>().as_slice() {
            [dir, val] => {
                for _ in 0..val.parse::<usize>().unwrap() {
                    current_head_pos = match *dir {
                        "L" => Position {
                            x: current_head_pos.x - 1,
                            y: current_head_pos.y,
                        },
                        "R" => Position {
                            x: current_head_pos.x + 1,
                            y: current_head_pos.y,
                        },
                        "U" => Position {
                            x: current_head_pos.x,
                            y: current_head_pos.y + 1,
                        },
                        "D" => Position {
                            x: current_head_pos.x,
                            y: current_head_pos.y - 1,
                        },
                        _ => current_head_pos,
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

fn calculate_new_tail(head: Position, tail: Position) -> Position {
    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;

    if x_diff.abs() > 1 {
        return Position {
            x: if x_diff > 1 {
                tail.x + 1
            } else if x_diff < -1 {
                tail.x - 1
            } else {
                tail.x
            },
            y: if y_diff > 0 {
                tail.y + 1
            } else if y_diff < 0 {
                tail.y - 1
            } else {
                tail.y
            },
        };
    } else if y_diff.abs() > 1 {
        return Position {
            x: if x_diff > 0 {
                tail.x + 1
            } else if x_diff < 0 {
                tail.x - 1
            } else {
                tail.x
            },
            y: if y_diff > 1 {
                tail.y + 1
            } else if y_diff < 1 {
                tail.y - 1
            } else {
                tail.y
            },
        };
    }
    tail
}
