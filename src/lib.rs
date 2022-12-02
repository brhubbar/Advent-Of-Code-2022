// Common functions for advent of code 2022.
use std::fs;
use std::collections::HashMap;


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
