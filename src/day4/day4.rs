use std::collections::HashSet;

pub fn problem1() {
    use std::fs;

    let contents = fs::read_to_string("src/day4/input.txt").expect("Should have found a file");

    let mut overlappers = 0;

    for pair in contents.split("\n") {
        let mut elf_ranges = Vec::new();

        for elf in pair.split(",") {
            if let &[begin, end] = elf.split("-").collect::<Vec<&str>>().as_slice() {
                let begin_range: i32 = begin.parse().unwrap();
                let end_range: i32 = end.parse().unwrap();
                let vals: HashSet<i32> = (begin_range..=end_range).collect();
                elf_ranges.push(vals);
            }
        }

        let intersection = elf_ranges[0].intersection(&elf_ranges[1]);
        let count = intersection.count();
        if  count == elf_ranges[0].len() || count == elf_ranges[1].len() {
            overlappers += 1;
        }

    }
    println!("Day 4, Problem 1: {:?}", overlappers);
}

pub fn problem2() {
    use std::fs;

    let contents = fs::read_to_string("src/day4/input.txt").expect("Should have found a file");

    let mut overlappers = 0;

    for pair in contents.split("\n") {
        let mut elf_ranges = Vec::new();

        for elf in pair.split(",") {
            if let &[begin, end] = elf.split("-").collect::<Vec<&str>>().as_slice() {
                let begin_range: i32 = begin.parse().unwrap();
                let end_range: i32 = end.parse().unwrap();
                let vals: HashSet<i32> = (begin_range..=end_range).collect();
                elf_ranges.push(vals);
            }
        }

        let intersection = elf_ranges[0].intersection(&elf_ranges[1]);
        let count = intersection.count();
        if  count > 0 {
            overlappers += 1;
        }

    }
    println!("Day 4, Problem 2: {:?}", overlappers);
}