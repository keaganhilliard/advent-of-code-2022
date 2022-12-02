use day1::day1::process_elves;
use day2::{problem1::calculate_score, problem2::calculate_score_again};

mod day1 {
    pub mod day1;
}

mod day2 {
    pub mod problem1;
    pub mod problem2;
}

fn main() {
    calculate_score_again();
}
