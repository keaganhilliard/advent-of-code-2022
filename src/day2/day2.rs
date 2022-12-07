use std::collections::HashMap;

pub fn problem1() {
    let values = HashMap::from([("A", 1), ("X", 1), ("B", 2), ("Y", 2), ("C", 3), ("Z", 3)]);

    let contents = include_str!("input.txt");

    let mut total_score = 0;
    for round in contents.split("\n") {
        if let &[opponent, player] = round.split(" ").collect::<Vec<&str>>().as_slice() {
            if let Some(opp_value) = values.get(opponent) {
                if let Some(player_value) = values.get(player) {
                    if *opp_value == *player_value {
                        total_score += 3 + player_value;
                    } else if (*opp_value == 1 && *player_value == 2)
                        || (*opp_value == 2 && *player_value == 3)
                        || (*opp_value == 3 && *player_value == 1)
                    {
                        total_score += 6 + player_value;
                    } else {
                        total_score += player_value;
                    }
                }
            }
        }
    }
    println!("Day 2, Problem 1: {}", total_score);
}

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

pub fn problem2() {
    let values = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);

    let contents = include_str!("input.txt");

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
    println!("Day 2, Problem 2: {}", total_score);
}
