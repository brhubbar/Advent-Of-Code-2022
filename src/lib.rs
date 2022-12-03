// Common functions for advent of code 2022.
use std::fs;
use std::collections::HashMap;
use array_tool::vec::Intersect;

/// Read the contents of a file directly into a String.
///
/// Parameters
/// ----------
/// file_path : &str
///     Path to read from.
///
pub fn read_file(file_path: &str) -> String {
    // source: https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

/// Find the mis-sorted contents of the rucksack.
///
/// Splits the string in half, then finds the common character between the
/// halves.
///
/// Assumes characters coming in are in the english alphabet; therefore ascii,
/// therefore 8-bit characters.
pub fn find_missort(sack_contents: &str) -> u8 {
    let n_items = sack_contents.len();  // Assumes 8-bit characters
    let mut split = sack_contents.as_bytes().chunks(n_items/2);
    let frontseat: Vec<u8> = split.next().unwrap().to_vec();
    let backseat: Vec<u8> = split.next().expect("Character unwrapping goofed up.").to_vec();
    let effed_up = frontseat.intersect(backseat);
    if effed_up.len() != 1 {
        panic!("<goat scream>... How bad are you at sorting?!")
    }
    effed_up[0]
}

pub fn identify_badge(sack_contents: &[&&str]) -> u8 {
    let mut badgaroni: Vec<u8> = (65..124).collect();
    for sack in sack_contents {
        let sack_bytes: Vec<u8> = sack.as_bytes().to_vec();
        badgaroni = badgaroni.intersect(sack_bytes);
    }
    if badgaroni.len() != 1 {
        panic!("<crying Cory>... you have one job! Find the badge!")
    }
    badgaroni[0]
}

/// Assign a priority to an item.
///
/// 'a' = 1 (UTF == 97)
/// 'A' = 27 (UTF == 65)
/// ...
pub fn prioritize_items(item: u8) -> u8 {
    if item >= 97 { item - 97 + 1}
    else { item - 65 + 27}
}

/// Return the score of a rock-paper-scissors round
///
/// Opponent Plays:
/// A = Rock
/// B = Paper
/// C = Scissors
///
/// Your Plays:
/// X = Rock
/// Y = Paper
/// Z = Scissors
///
/// Scoring
/// Rock: +1
/// Paper: +2
/// Scissors: +3
/// Draw: +3
/// Win: +6
pub fn rps_explicit(pair: &str) -> u8 {
    let mut results: HashMap<&str, u8> = HashMap::new();
    results.insert("A X", 4);  // X = 1, draw = 3
    results.insert("A Y", 8);  // Y = 2, win = 6
    results.insert("A Z", 3);  // Z = 3
    results.insert("B X", 1);  // X = 1
    results.insert("B Y", 5);  // Y = 2, draw = 3
    results.insert("B Z", 9);  // Z = 3, win = 6
    results.insert("C X", 7);  // X = 1, win = 6
    results.insert("C Y", 2);  // Y = 2
    results.insert("C Z", 6);  // Z = 3, draw = 3

    results[pair]
}

/// Return the score of a rock-paper-scissors round. In this case, you are being
/// commanded how to finish the round, instead of what to play.
///
/// Opponent Plays:
/// A = Rock
/// B = Paper
/// C = Scissors
///
/// Your Plays:
/// X = lose
/// Y = draw
/// Z = win
///
/// Scoring
/// Rock: +1
/// Paper: +2
/// Scissors: +3
/// Draw: +3
/// Win: +6
pub fn rps_implicit(pair: &str) -> u8 {
    let mut results: HashMap<&str, u8> = HashMap::new();
    results.insert("A X", 3);  // lose = 0, scissors = 3
    results.insert("A Y", 4);  // draw = 3, rock = 1
    results.insert("A Z", 8);  // win = 6, paper = 2
    results.insert("B X", 1);  // lose = 0, rock = 1
    results.insert("B Y", 5);  // draw = 3, paper = 2
    results.insert("B Z", 9);  // win = 6, scissors = 3
    results.insert("C X", 2);  // lose = 0, paper = 2
    results.insert("C Y", 6);  // draw = 3, scissors = 3
    results.insert("C Z", 7);  // win = 6, rock = 1

    results[pair]
}
