use std::{
    collections::{HashSet, VecDeque},
    fs,
};

pub fn problem1() {
    let contents = fs::read_to_string("src/day6/input.txt").expect("Should have found a file");
    println!("Day 6, Problem 1: {}", index_after_unique(&contents, 4));
}

pub fn problem2() {
    let contents = fs::read_to_string("src/day6/input.txt").expect("Should have found a file");
    println!("Day 6, Problem 2: {}", index_after_unique(&contents, 14));
}

fn index_after_unique(contents: &str, limit: usize) -> usize {
    let mut char_set = VecDeque::new();
    for (i, c) in contents.char_indices() {
        char_set.push_back(c);
        let len = char_set.len();
        if len == limit {
            if len == HashSet::<char>::from_iter(char_set.iter().copied()).len() {
                return i + 1;
            }
            char_set.pop_front().unwrap();
        }
    }
    0
}

#[test]
fn problem1_test() {
    let key_length = 4;

    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let expected = 7;
    let actual = index_after_unique(input, key_length);
    assert_eq!(actual, expected);

    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let expected = 5;
    let actual = index_after_unique(input, key_length);
    assert_eq!(actual, expected);

    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    let expected = 6;
    let actual = index_after_unique(input, key_length);
    assert_eq!(actual, expected);

    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let expected = 10;
    let actual = index_after_unique(input, key_length);
    assert_eq!(actual, expected);

    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let expected = 11;
    let actual = index_after_unique(input, key_length);
    assert_eq!(actual, expected);
}

#[test]
fn problem2_test() {
    let key_length = 14;

    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let expected = 19;
    let actual = index_after_unique(input, key_length);
    assert_eq!(actual, expected);

    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let expected = 23;
    let actual = index_after_unique(input, key_length);
    assert_eq!(actual, expected);

    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    let expected = 23;
    let actual = index_after_unique(input, key_length);
    assert_eq!(actual, expected);

    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let expected = 29;
    let actual = index_after_unique(input, key_length);
    assert_eq!(actual, expected);

    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let expected = 26;
    let actual = index_after_unique(input, key_length);
    assert_eq!(actual, expected);
}
