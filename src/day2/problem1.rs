/*
Opponent
    A = Rock
    B = Paper
    C = Scissors

Me
    X = Rock
    Y = Paper
    Z = Scissors
*/

use std::collections::HashMap;

pub fn calculate_score() {
    use std::fs;
    let values = HashMap::from([("A", 1), ("X", 1), ("B", 2), ("Y", 2), ("C", 3), ("Z", 3)]);

    let contents = fs::read_to_string("src/day2/input.txt").expect("Should have found a file");

    let mut total_score = 0;
    for round in contents.split("\n") {
        if let &[opponent, player] = round.split(" ").collect::<Vec<&str>>().as_slice() {
            if let Some(opp_value) = values.get(opponent) {
                if let Some(player_value) = values.get(player) {
                    let mut current_score = 0;
                    if *opp_value == *player_value {
                        total_score += 3 + player_value;
                        current_score = 3 + player_value;
                    } else if (*opp_value == 1 && *player_value == 2)
                        || (*opp_value == 2 && *player_value == 3)
                        || (*opp_value == 3 && *player_value == 1)
                    {
                        total_score += 6 + player_value;
                        current_score = 6 + player_value;
                    } else {
                        total_score += player_value;
                        current_score = *player_value;
                    }
                    // println!(
                    //     "Opponent: {}, Player: {}, Outcome: {}",
                    //     opponent, player, current_score
                    // );
                }
            }
        }
    }
    println!("Total score: {}", total_score);
}
