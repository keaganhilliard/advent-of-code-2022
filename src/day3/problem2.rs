use std::collections::HashSet;

pub fn get_badge() {
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
        let intersection = iter.fold(base, |set1, set2| set1.intersection(set2).copied().collect());

        for c in intersection {
            if let Some(priority) = priorities.find(&c.to_string()) {
                sum += priority;
            }
        }

    }
    
    println!("{:?}", sum);

}