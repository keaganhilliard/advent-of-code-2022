#[derive(Debug)]
struct Elf {
    calories: i32,
}

fn get_elves() -> Vec<Elf> {
    let contents = include_str!("input.txt");

    let mut elves: Vec<Elf> = Vec::new();

    for elf_str in contents.split("\n\n") {
        let mut total_calories = 0;
        for calories in elf_str.split("\n") {
            if let Ok(p) = calories.parse::<i32>() {
                total_calories += p;
            }
        }
        elves.push(Elf {
            calories: total_calories,
        })
    }

    elves.sort_by(|a, b| b.calories.partial_cmp(&a.calories).unwrap());

    return elves;
}

pub fn problem1() {
    println!("Day 1, Problem 1: {:?}", get_elves()[0].calories);
}

pub fn problem2() {
    let mut top3 = 0;
    for elf in &get_elves()[0..3] {
        top3 += elf.calories;
    }
    println!("Day 1, Problem 2: {:?}", top3);
}
