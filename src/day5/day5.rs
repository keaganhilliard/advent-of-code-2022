use std::{fs, collections::VecDeque};

use regex::Regex;

pub fn problem1() {
    let contents = fs::read_to_string("src/day5/input.txt").expect("Should have found a file");

    let mut stack_vec = vec![VecDeque::new(),VecDeque::new(),VecDeque::new(),VecDeque::new(),VecDeque::new(),VecDeque::new(),VecDeque::new(),VecDeque::new(),VecDeque::new()];
    let move_regex = Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();

    if let &[stacks, moves] = contents.split("\n\n").collect::<Vec<&str>>().as_slice() {
        let mut current_stack_index = 0;
        for row in stacks.split("\n") {
            let mut position = 0;
            for c in row.chars() {
                let mut current_vec = &mut stack_vec[current_stack_index];
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
            let (count, from, to) = (&caps["count"].parse::<usize>().unwrap(), &caps["from"].parse::<usize>().unwrap(), &caps["to"].parse::<usize>().unwrap());

            for i in 0..*count {
                let mut val: Option<char> = None;
                {
                    let from_vec = &mut stack_vec[*from-1];
                    val = from_vec.pop_back();
                }
                let to_vec = &mut stack_vec[*to-1];
                to_vec.push_back(val.unwrap());

            }
        }
    }

    let mut message_vec = Vec::new();
    for d in stack_vec {
        if let Some(c) = d.back() {
            message_vec.push(*c);
        }
    }
    let message = message_vec.iter().copied().collect::<String>();
    println!("Day 5, Problem 1: {}", message);

}