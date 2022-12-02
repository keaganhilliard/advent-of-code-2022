#[derive(Debug)]
struct Elf {
    num: i32,
    calories: i32,
}

pub fn process_elves() {
    use std::fs;
    let contents = fs::read_to_string("src/day1/input.txt").expect("Should have found a file");

    let mut elves: Vec<Elf> = Vec::new();
    let mut i = 0;

    for elf_str in contents.split("\n\n") {
        i += 1;
        let mut total_calories = 0;
        for calories in elf_str.split("\n") {
            if let Ok(p) = calories.parse::<i32>() {
                total_calories += p;
            }
        }
        elves.push(Elf {
            num: i,
            calories: total_calories,
        })
    }

    elves.sort_by(|a, b| b.calories.partial_cmp(&a.calories).unwrap());

    let mut top3 = 0;
    for elf in &elves[0..3] {
        top3 += elf.calories;
    }
    println!("Top: {:?}", elves[0]);
    println!("Top3: {:?}", top3);
}
