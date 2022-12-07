use std::{
    collections::{HashMap, VecDeque},
    fs,
};

pub fn problem1() {
    let contents = fs::read_to_string("src/day7/input.txt").expect("Should have found a file");
    let dirs = process_commands(&contents);
    let mut sum = 0;
    for (_, size) in dirs {
        if size <= 100000 {
            sum += size;
        }
    }
    println!("Day 7, Problem 1: {}", sum);
}

pub fn problem2() {
    let contents = fs::read_to_string("src/day7/input.txt").expect("Should have found a file");
    let dirs = process_commands(&contents);
    let mut best_dir = 0;
    let total_space_used = dirs.get("/").unwrap();
    let total_unused_space = 70000000 - total_space_used;
    let total_space_needed = 30000000 - total_unused_space;
    for (_, size) in dirs {
        if size > total_space_needed {
            if best_dir == 0 {
                best_dir = size;
            }
            if size < best_dir {
                best_dir = size;
            }
        }
    }
    println!("Day 7, Problem 2: {}", best_dir);
}

fn process_commands(contents: &str) -> HashMap<String, i32> {
    let mut dir_vec = VecDeque::new();
    let mut dirs = HashMap::new();
    let mut current_dir = String::from("");
    for command in contents.split("\n") {
        match command.split(" ").collect::<Vec<&str>>().as_slice() {
            &["$", "cd", ".."] => {
                current_dir = dir_vec.pop_back().unwrap();
            }
            &["$", "cd", path] => {
                if current_dir != "" {
                    dir_vec.push_back(current_dir.clone());
                    dirs.entry(current_dir.clone()).or_insert(0);
                }
                if path == "/" {
                    current_dir = "/".to_string();
                } else if current_dir == "/" {
                    current_dir = vec![current_dir.clone(), path.to_string()].join("").into();
                } else {
                    current_dir = vec![current_dir.clone(), path.to_string()].join("/").into();
                }
            }
            &["$", "ls"] => {}
            &["dir", _] => {}
            &[file_size, _] => {
                *dirs.entry(current_dir.clone()).or_insert(0) += file_size.parse::<i32>().unwrap();
                for dir in &dir_vec {
                    *dirs.entry(dir.clone()).or_insert(0) += file_size.parse::<i32>().unwrap();
                }
            }
            _ => {}
        }
    }
    dirs
}
