use std::{
    collections::{HashSet, VecDeque},
    fs,
};

pub fn problem1() {
    let contents = fs::read_to_string("src/day6/input.txt").expect("Should have found a file");
    println!("Day 6, Problem 1: {}", get_start(4));
}

pub fn problem2() {
    let contents = fs::read_to_string("src/day6/input.txt").expect("Should have found a file");
    println!("Day 6, Problem 2: {}", get_start(14));
}

fn get_start(limit: usize) -> usize {
    let contents = fs::read_to_string("src/day6/input.txt").expect("Should have found a file");
    let mut char_set = VecDeque::new();
    for (i, c) in contents.char_indices() {
        char_set.push_back(c);
        let len = char_set.len();
        if len == limit {
            if len == HashSet::<char>::from_iter(char_set.iter().copied()).len() {
                println!("Time elapsed: {:?}", now.elapsed());
                return i + 1;
            }
            char_set.pop_front().unwrap();
        }
    }
    0
}
