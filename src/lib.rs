// Common functions for advent of code 2022.
use std::fs;
use std::collections::{
    HashMap,
    VecDeque,
};

use array_tool::vec::{
    Intersect,
    Uniq,
};
use regex::Regex;

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

/// Parse bash interactions.
///
/// Do them one at a time (input and output), accepting the current directory
/// and the command. Return the current directory and update the borrowed file
/// tree.
///
/// CWD
/// Owners of each file
/// Size of each file
pub fn parse_comms_comms<'a>(
    mut cwd: String,
    cmd: &'a str,
    file_owners: &mut HashMap<String, Vec<String>>,
    file_sizes: &mut HashMap<String, usize>,
) -> String {
    if cmd.trim().starts_with("cd") {
        let dir = cmd.trim().strip_prefix("cd ").expect("You are no good at parsing.");
        if dir == "/" {
            return "/".to_string()
        }
        if dir == ".." {
            // This is dumb, but it'll work.
            cwd = cwd.trim_end_matches('/').to_string();
            let offset = cwd.rfind('/').unwrap();
            cwd.truncate(offset);  // Keep the trailing '/'
            return cwd + "/"
        }
        return cwd + dir + "/";  // One has to be owned - the other is borrowed.
    }

    // Command must start with ls.
    if !cmd.trim().starts_with("ls") {
        println!("{:?}", cmd);
        panic!("Unsupported command");
    }
    let parts: Vec<&str> = cmd.split('\n').map(str::trim).collect();
    for part in parts {
        if part == "ls" {
            continue
        }
        // println!("{:?}", part);
        let (type_or_size, name) = part.split_once(' ').unwrap();
        if type_or_size == "dir" {
            // I don't need to do anything with this.
            continue
        }
        let size = type_or_size.parse::<usize>().expect("Not a number!?");
        file_sizes.insert(name.to_owned(), size);
        let mut parents: Vec<String> = Vec::new();
        for parent in cwd.split('/') {
            if parent.trim().is_empty() {
                continue
            }
            parents.push(parent.to_owned());
        }
        file_owners.insert(name.to_owned(), parents);
    }

    cwd
}

/// Find the start of a packet.
///
/// The location of the last character in a block of 4 unique characters.
///
/// Plan is to use a VecDeque to scroll through the string, then use
/// array_tools:Vec:Uniq to find when all four characters are different.
pub fn find_marker(datastream: &str, marker_size: usize) -> usize {
    let mut buffer: VecDeque<u8> = VecDeque::with_capacity(marker_size);
    for (marker_location, character) in datastream.as_bytes().iter().enumerate() {
        if buffer.len() < marker_size {
            buffer.push_back(*character);
            continue;
        }
        let unique_check = Vec::from_iter(buffer.iter());
        if unique_check.is_unique() {
            return marker_location
        }
        buffer.pop_front();
        buffer.push_back(*character);
    }

    panic!("Didn't find the packet marker!")
}

/// Parse crates arrangement.
///
/// This'll rearrange the visual stacks into vectors that I can push and pop
/// from. I need LIFO (stack) effect. Vec offers this with pop_back()
/// (https://stackoverflow.com/a/40851723).
///
/// Each column is a stack, which kinda blows, not gonna lie. What I'll do is
/// split in chunks of 4 characters. The last one will be 3 characters long
/// since the newline will have been dumped. I can then .extend() each stack
/// with the returned empty or single-value vec. Other option is to use regex on
/// each row. I actually like that better... time to learn regex for rust.
pub fn create_stack_regex(crate_labels: &str) -> Vec<Regex> {
    println!("{:?}", crate_labels);
    let mut finders: Vec<Regex> = Vec::new();
    for i in (0..crate_labels.len()).step_by(4) {
        let expression = format!(r"[\n^]+.{{{}}}\[(\w)\]", i);
        finders.push(Regex::new(expression.as_str()).expect("Write better expressions, my dude."));
    }
    println!("{:?}", finders);
    finders
}

/// Rearrange crates as commanded.
///
/// Command format is move a from b to c
pub fn make_moves(moves: &str, stacks: &mut [Vec<&str>]) {
    let expression = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let captures = expression.captures(moves).expect("Couldn't recognize the command.");
    let n = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
    let to = captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
    let mut moving_stack: Vec<&str> = Vec::new();
    for _ in 0..n {
        moving_stack.push(stacks[from].pop().expect("The `from` stack is emptry!"));
    }
    // Comment out for part 1
    moving_stack.reverse();
    stacks[to].extend(moving_stack);
}

/// Find ranges that fully contain other ranges.
///
/// Input format: 'lower-upper,lower-upper'
pub fn is_full_overlap(assignment_set: &str) -> bool {
    let mut assignments: Vec<u32> = Vec::new();
    for assignment in assignment_set.split(',') {
        for bound in assignment.split('-') {
            assignments.push(bound.parse::<u32>().expect("Failed to parse a number outta that, boy-o."))
        }
    }
    let max = assignments.iter().max().unwrap();
    let min = assignments.iter().min().unwrap();
    for assignment in assignments.chunks(2) {
        if assignment.contains(min) && assignment.contains(max) {
            return true
        }
    }
    false
}

/// Assumes there are only two pairs in the set
pub fn is_partial_overlap(assignment_set: &str) -> bool {
    let mut assignments: Vec<u32> = Vec::new();
    for assignment in assignment_set.split(',') {
        for bound in assignment.split('-') {
            assignments.push(bound.parse::<u32>().expect("Failed to parse a number outta that, boy-o."))
        }
    }
    // https://stackoverflow.com/a/325964
    assignments[0] <= assignments[3] && assignments[1] >= assignments[2]
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
