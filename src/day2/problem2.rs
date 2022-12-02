/*
Opponent
    A = Rock
    B = Paper
    C = Scissors

Me
    X = Lose
    Y = Draw
    Z = Win
*/
use std::str::FromStr;

enum WhatToDo {
    Win,
    Lose,
    Draw,
}

impl FromStr for WhatToDo {
    type Err = ();

    fn from_str(input: &str) -> Result<WhatToDo, Self::Err> {
        match input {
            "X" => Ok(WhatToDo::Lose),
            "Y" => Ok(WhatToDo::Draw),
            "Z" => Ok(WhatToDo::Win),
            _ => Err(()),
        }
    }
}

use std::collections::HashMap;

pub fn calculate_score_again() {
    use std::fs;
    let values = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);

    let contents = fs::read_to_string("src/day2/input.txt").expect("Should have found a file");

    let mut total_score = 0;
    for round in contents.split("\n") {
        if let &[opponent, do_this] = round.split(" ").collect::<Vec<&str>>().as_slice() {
            if let Some(opponent_value) = values.get(opponent) {
                match WhatToDo::from_str(do_this) {
                    Ok(WhatToDo::Lose) => match opponent_value {
                        1 => total_score += 3,
                        2 => total_score += 1,
                        3 => total_score += 2,
                        _ => {}
                    },
                    Ok(WhatToDo::Win) => match opponent_value {
                        1 => total_score += 2 + 6,
                        2 => total_score += 3 + 6,
                        3 => total_score += 1 + 6,
                        _ => {}
                    },
                    Ok(WhatToDo::Draw) => total_score += opponent_value + 3,
                    _ => {}
                }
            }
        }
    }
    println!("Total score: {}", total_score);
}
