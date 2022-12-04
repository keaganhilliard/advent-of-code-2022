use std::collections::HashSet;

pub fn sum_priorities() {
    let priorities = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    use std::fs;

    let contents = fs::read_to_string("src/day3/input.txt").expect("Should have found a file");
    let mut dups = Vec::new();
    
    for sack in contents.split("\n") {
        let mid = sack.len()/2;
        
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
    println!("{:?}", dups);
    let mut sum = 0;
    for c in dups {
        if let Some(priority) = priorities.find(&c.to_string()) {
            sum += priority;
        }
    }
    println!("{:?}", sum);

}