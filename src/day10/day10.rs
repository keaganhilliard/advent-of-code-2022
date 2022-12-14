pub fn problem1() {
    let contents = include_str!("input.txt");

    let mut cycle = 0;
    let mut x = 1;
    let targets = vec![20, 60, 100, 140, 180, 220];
    let mut total_thing = 0;
    for op in contents.split("\n") {
        match op.split(" ").collect::<Vec<&str>>().as_slice() {
            ["addx", val] => {
                cycle += 1;
                if targets.contains(&cycle) {
                    total_thing += cycle * x
                }
                cycle += 1;
                if targets.contains(&cycle) {
                    total_thing += cycle * x
                }
                x += val.parse::<i32>().unwrap();
            }
            ["noop"] => {
                cycle += 1;
                if targets.contains(&cycle) {
                    total_thing += cycle * x
                }
            }
            _ => {}
        }
    }
    println!("Day 10, Problem 1: {}", total_thing);
}

pub fn problem2() {
    let contents = include_str!("input.txt");

    let mut cycle: usize = 0;
    let mut x: i32 = 1;
    let mut rows = vec![vec![" "; 40]; 6];
    for op in contents.split("\n") {
        match op.split(" ").collect::<Vec<&str>>().as_slice() {
            ["addx", val] => {
                draw(&mut rows, cycle, x);
                cycle += 1;
                draw(&mut rows, cycle, x);
                cycle += 1;
                x += val.parse::<i32>().unwrap();
            }
            ["noop"] => {
                draw(&mut rows, cycle, x);
                cycle += 1;
            }
            _ => {}
        }
    }
    println!("Day 10, Problem 2:");
    for line in rows {
        println!("{}", line.join(""));
    }
}

fn draw(rows: &mut Vec<Vec<&str>>, cycle: usize, x: i32) {
    let col = (cycle % 40) as i32;
    if col >= x - 1 && col <= x + 1 {
        rows[cycle / 40][cycle % 40] = "#";
    }
}
