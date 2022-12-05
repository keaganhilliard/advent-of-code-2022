use std::collections::HashSet;

pub fn problem1() {
    let priorities = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    use std::fs;

    let contents = fs::read_to_string("src/day3/input.txt").expect("Should have found a file");
    let mut dups = Vec::new();

    for sack in contents.split("\n") {
        let mid = sack.len() / 2;

        let mut first_compartment = Vec::new();
        let mut second_compartment = Vec::new();
        let mut both = HashSet::new();

        for (i, c) in sack.char_indices() {
            if i < mid {
                first_compartment.push(c);
            } else {
                if first_compartment.contains(&c) {
                    both.insert(c);
                }
                second_compartment.push(c)
            }
        }

        for c in both {
            dups.push(c);
        }
    }

    let mut sum = 0;
    for c in dups {
        if let Some(priority) = priorities.find(&c.to_string()) {
            sum += priority;
        }
    }
    println!("Day 3, Problem 1: {:?}", sum);
}

pub fn problem2() {
    let priorities = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    use std::fs;

    let contents = fs::read_to_string("src/day3/input.txt").expect("Should have found a file");
    let mut sum = 0;

    let sacks: Vec<&str> = contents.split("\n").collect();

    for group in sacks.chunks(3) {
        let mut elves: Vec<HashSet<_>> = Vec::new();
        for elf in group {
            let mut elf_set = HashSet::new();
            for c in elf.chars() {
                elf_set.insert(c);
            }
            elves.push(elf_set);
        }

        let mut iter = elves.iter();
        let base = iter.next().unwrap().clone();
        let intersection = iter.fold(base, |set1, set2| {
            set1.intersection(set2).copied().collect()
        });

        for c in intersection {
            if let Some(priority) = priorities.find(&c.to_string()) {
                sum += priority;
            }
        }
    }

    println!("Day 3, Problem 2: {:?}", sum);
}
