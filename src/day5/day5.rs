use std::{collections::VecDeque, fs};

use regex::Regex;

pub fn problem1() {
    let contents = fs::read_to_string("src/day5/input.txt").expect("Should have found a file");

    let mut stack_vec = Vec::new();
    let move_regex = Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();

    if let &[stacks, moves] = contents.split("\n\n").collect::<Vec<&str>>().as_slice() {
        let mut current_stack_index = 0;
        for row in stacks.split("\n") {
            let mut position = 0;
            for c in row.chars() {
                if current_stack_index == stack_vec.len() {
                    stack_vec.push(VecDeque::new());
                }
                let current_vec = &mut stack_vec[current_stack_index];
                position += 1;
                if c.is_alphabetic() {
                    current_vec.push_front(c);
                }
                if position == 4 {
                    current_stack_index += 1;
                    position = 0;
                }
            }
            current_stack_index = 0;
        }
        for move_crate in moves.split("\n") {
            let caps = move_regex.captures(move_crate).unwrap();
            let (count, from, to) = (
                *&caps["count"].parse::<usize>().unwrap(),
                *&caps["from"].parse::<usize>().unwrap(),
                *&caps["to"].parse::<usize>().unwrap(),
            );

            for _ in 0..count {
                let val = stack_vec[from - 1].pop_back().unwrap();
                stack_vec[to - 1].push_back(val);
            }
        }
    }

    let mut message_vec = Vec::new();
    for d in stack_vec {
        if let Some(c) = d.back() {
            message_vec.push(*c);
        }
    }
    println!(
        "Day 5, Problem 1: {}",
        message_vec.iter().copied().collect::<String>()
    );
}

pub fn problem2() {
    let contents = fs::read_to_string("src/day5/input.txt").expect("Should have found a file");

    let mut stack_vec = Vec::new();
    let move_regex = Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();

    if let &[stacks, moves] = contents.split("\n\n").collect::<Vec<&str>>().as_slice() {
        let mut current_stack_index = 0;
        for row in stacks.split("\n") {
            let mut position = 0;
            for c in row.chars() {
                if current_stack_index == stack_vec.len() {
                    stack_vec.push(VecDeque::new());
                }
                let current_vec = &mut stack_vec[current_stack_index];
                position += 1;
                if c.is_alphabetic() {
                    current_vec.push_front(c);
                }
                if position == 4 {
                    current_stack_index += 1;
                    position = 0;
                }
            }
            current_stack_index = 0;
        }
        for move_crate in moves.split("\n") {
            let caps = move_regex.captures(move_crate).unwrap();
            let (count, from, to) = (
                *&caps["count"].parse::<usize>().unwrap(),
                *&caps["from"].parse::<usize>().unwrap(),
                *&caps["to"].parse::<usize>().unwrap(),
            );

            let mut moves = VecDeque::new();
            for _ in 0..count {
                let val = stack_vec[from - 1].pop_back().unwrap();
                moves.push_front(val);
            }
            for val in moves {
                let to_vec = &mut stack_vec[to - 1];
                to_vec.push_back(val);
            }
        }
    }

    let mut message_vec = Vec::new();
    for d in stack_vec {
        if let Some(c) = d.back() {
            message_vec.push(*c);
        }
    }
    println!(
        "Day 5, Problem 2: {}",
        message_vec.iter().copied().collect::<String>()
    );
}
