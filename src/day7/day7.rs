use std::collections::{HashMap, VecDeque};

pub fn problem1() {
    let contents = include_str!("input.txt");
    let dirs = process_commands(&contents);

    println!(
        "Day 7, Problem 1: {}",
        get_total_less_than_limit(dirs, 100000)
    );
}

pub fn problem2() {
    let contents = include_str!("input.txt");
    let dirs = process_commands(&contents);

    println!(
        "Day 7, Problem 2: {}",
        get_space_to_delete(dirs, 70000000, 30000000)
    );
}

fn get_total_less_than_limit(dirs: HashMap<String, i32>, limit: i32) -> i32 {
    let mut sum = 0;
    for (_, size) in dirs {
        if size <= limit {
            sum += size;
        }
    }
    sum
}

fn get_space_to_delete(dirs: HashMap<String, i32>, total_space: i32, needed_space: i32) -> i32 {
    let mut best_dir = 0;
    let total_space_used = dirs.get("/").unwrap();
    let total_unused_space = total_space - total_space_used;
    let total_space_needed = needed_space - total_unused_space;
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
    best_dir
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

#[test]
fn problem1_test() {
    let contents = include_str!("tester.txt");
    let dirs = process_commands(&contents);
    assert_eq!(95437, get_total_less_than_limit(dirs, 100000));
}

#[test]
fn problem2_test() {
    let contents = include_str!("tester.txt");
    let dirs = process_commands(&contents);
    assert_eq!(24933642, get_space_to_delete(dirs, 70000000, 30000000));
}
