use std::cmp::Ordering;
// Common functions for advent of code 2022.
use std::{
    fs,
};
use std::collections::{
    HashMap,
    HashSet,
    VecDeque,
};

use array_tool::vec::{
    Intersect,
    Uniq,
};
use itertools::Itertools;
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

/// Day 14: Sand tracking.
///
/// - Sand is falling into a cave.
/// - Cave spaces can be Sand, Rock, or Air.
/// - Sand falls until it rests. Its fall behavior is as follows:
///     - Fall downward if occupying air, else
///     - Fall diagonal down-left if occupying air, else
///     - Fall diagonal down-right if occupying air, else
///     - Rest

#[derive(Clone, Copy, Debug)]
enum SpaceType {
    Air,
    Rock,
    Sand,
}


/// Tracks spaces that are not air.
pub struct Cave {
    spaces: HashMap<[isize; 2], SpaceType>,
    sand_source: [isize; 2],
}

impl Cave {
    pub fn new(sand_source: [isize; 2]) -> Self {
        Self {
            spaces: HashMap::new(),
            sand_source,
        }
    }

    /// Return [[L, R], [B, T]]
    fn get_bounds(&self) -> [[isize; 2]; 2] {
        let l = self.spaces
            .keys()
            .map(|[x, _]| *x)
            .min()
            .unwrap();
        let r = self.spaces
            .keys()
            .map(|[x, _]| *x)
            .max()
            .unwrap();
        let t = self.spaces
            .keys()
            .map(|[_, y]| *y)
            .min()
            .unwrap();
        let b = self.spaces
            .keys()
            .map(|[_, y]| *y)
            .max()
            .unwrap();
        [[l, r], [b, t]]
    }

    pub fn print_cave(&self) {
        let bounds = self.get_bounds();
        // println!("{bounds:?}");
        for y in 0..=bounds[1][0] {
            for x in bounds[0][0]..=bounds[0][1] {
                // print!("({x}, {y}); ");
                if [x, y] == [500, 0] {
                    print!("+");
                    continue
                }
                match self.get_space(&[x, y]).expect("Bad iterator for printing.") {
                    SpaceType::Rock => print!("#"),
                    SpaceType::Sand => print!("o"),
                    SpaceType::Air => print!("."),
                }
            }
            println!();
        }
    }

    fn get_space(&self, coords: &[isize; 2]) -> Option<SpaceType>{
        if let Some(space) = self.spaces.get(coords) {
            Some(space.to_owned())
        } else {
            let bounds = self.get_bounds();
            // println!("{coords:?} within {bounds:?}?");
            if coords[0] < bounds[0][0]
                || coords[0] > bounds[0][1]
                || coords[1] > bounds[1][0]  // Y is inverted
                // || coords[1] > bounds[1][1]  // This is checking if the sand
                // is too high, which doesn't apply
            {
                return None
            }
            Some(SpaceType::Air)
        }
    }

    pub fn read_scan(&mut self, scan: &str) {
        for edge in scan.split('\n') {
            let mut vertices: Vec<[isize; 2]> = Vec::new();
            for vertex in edge.split(" -> ") {
                let coords: Vec<isize> = vertex
                    .split(',')
                    .map(|coord| coord.parse::<isize>().expect("Not an integer!"))
                    .collect();
                vertices.push([coords[0], coords[1]])
            }
            let mut last_vertex: [isize; 2] = vertices.pop().expect("Empty edge!");
            let mut current_vertex: [isize; 2];
            while !vertices.is_empty() {
                current_vertex = vertices.pop().expect("Your while loop STANKS");
                // println!("{last_vertex:?} -> {current_vertex:?}");
                let mut x: [isize; 2] = [last_vertex[0], current_vertex[0]];
                let mut y: [isize; 2] = [last_vertex[1], current_vertex[1]];
                x.sort();
                y.sort();
                if x[0] == x[1] {
                    // Vertical edge.
                    // This will overwrite every vertex except the first.
                    for y in y[0]..=y[1] {
                        self.spaces.insert([x[0], y], SpaceType::Rock);
                    }
                } else if y[0] == y[1] {
                    // Vertical edge.
                    // This will overwrite every vertex except the first.
                    for x in x[0]..=x[1] {
                        self.spaces.insert([x, y[0]], SpaceType::Rock);
                    }
                } else {
                    panic!("Overlapping vertices!")
                }
                last_vertex = current_vertex;
            }
        }
    }

    /// Add a grain of sand and place it at rest. Returns true if the sand came
    /// to rest, false if the sand fell out into the abyss.
    pub fn add_grain_of_sand(&mut self) -> bool {
        // Calculate where it'll end up and then place it directly, rather than
        // moving it through space.
        // println!("{:?}", self.spaces);
        let mut sand_resting_place: [isize; 2] = self.sand_source;
        'falling_sand: loop {
            let [x, y]: [isize; 2] = sand_resting_place;
            let try_spaces: Vec<[isize; 2]> = vec![[x, y+1], [x-1, y+1], [x+1, y+1]];
            for try_space in try_spaces {
                match self.get_space(&try_space) {
                    Some(SpaceType::Air) => {
                        sand_resting_place = try_space;
                        continue 'falling_sand
                    },
                    None => {
                        // The next space is off the map, sand falls into that
                        // space for eternity, so send a signal to indicate
                        // that.
                        return false
                    },
                    _ => {},
                }
            }
            // Stopped falling.
            self.spaces.insert(sand_resting_place, SpaceType::Sand);
            return true

        }
    }
}

impl Default for Cave {
    fn default() -> Self {
        Self::new([500, 0])
    }
}

/// Packet sorting
///
/// Each line is a list of integers and lists, forming a packet. Compare values
/// between the two packets.
///
/// - Comparing integers: **lower should come first**.
/// - Comparing lists: iterate through lists **comparing values**. If a list
///   comes to an end before a decision is made, **the shorter list should come
///   first**. equal length lists tell you nothing.
/// - Comparing a list and an integer: convert int to list, then compare lists.
///
/// Returns true if in correct order.

#[derive(Debug)]
enum PacketValue {
    Integer(usize),
    List(String),
    None,
}

pub fn compare_lists(left: String, right: String) -> Ordering {
    // print!("{left}, {right} => ");
    let (left, next_left_value) = get_next_value(left);
    let (right, next_right_value) = get_next_value(right);
    // println!("{next_left_value:?}, {next_right_value:?}");
    // Compare current values.
    let result: Ordering = match next_left_value {
        PacketValue::Integer(left) => {
            // Compare to right.
            match next_right_value {
                PacketValue::Integer(right) => {
                    // println!("Path 1");
                    left.cmp(&right)
                },
                PacketValue::List(right) => {
                    // println!("Path 2");
                    compare_lists(left.to_string(), right)
                }
                PacketValue::None => {
                    // Right list is empty and Left list is not.
                    Ordering::Greater
                }
            }
        },
        PacketValue::List(left) => {
            // Recurse.
            match next_right_value {
                PacketValue::Integer(right) => {
                    // println!("Path 3");
                    compare_lists(left, right.to_string())
                },
                PacketValue::List(right) => {
                    // println!("Path 4");
                    compare_lists(left, right)
                },
                PacketValue::None => {
                    // Right list is empty and left list might be.
                    compare_lists(left, "".to_string())
                },
            }
        },
        PacketValue::None => {
            match next_right_value {
                PacketValue::Integer(_) => {
                    // Left list is empty and right list is not, so sort is
                    // correct.
                    Ordering::Less
                },
                PacketValue::List(right) => {
                    // println!("Path 4");
                    // Another 'might be' case.
                    compare_lists("".to_string(), right)
                },
                PacketValue::None => {
                    // Both lists are empty lists, so nothing decisive.
                    Ordering::Equal
                },
            }
        }
    };

    // println!("Result: {result:?}; {left:?}, {right:?}");

    if result == Ordering::Equal {
        match left {
            Some(left) => {
                // Left has a remaining value.
                match right {
                    Some(right) => {
                        compare_lists(left, right)
                    }
                    None => {
                        // Left is longer than right.
                        Ordering::Greater
                    }
                }
            },
            None => {
                // Left is done.
                // If right still exists, then left is shorter than right,
                // which means they are properly sorted.
                // If right is also done, then Left == Right.
                match right {
                    Some(_) => Ordering::Less,
                    None => Ordering::Equal,
                }
            }
        }
    } else {
        result
    }
}


/// Returns the list with the 'popped' value removed, plus the popped value.
/// Assumes that the outermost list, so to speak, has its brackets removed. If
/// they're not, no need to fret: it'll just return that outermost list.
/// If it determines that it's returning the last value in the list, it'll
/// return None for the remaining list.
fn get_next_value(list: String) -> (Option<String>, PacketValue) {
    if list.starts_with('[') {
        // Break out the outermost list and return it.
        let mut closing_brace_idx: usize = list.len();
        let mut depth_counter = 0;
        for (idx, c) in list.chars().enumerate() {
            match c {
                '[' => depth_counter += 1,  // Guaranteed to happen on first iteration.
                ']' => depth_counter -= 1,
                _ => {},
            }

            if depth_counter == 0 {
                closing_brace_idx = idx;
                break
            }
        }

        // Get the next value, a list, without its braces.
        let popped_list: String = list.chars()
            .enumerate()
            .filter(|(idx, _)| *idx > 0 && *idx < closing_brace_idx)
            .map(|(_, c)| c)
            .collect();
        // Get the list with the next value removed.
        let remaining_list: String = list.chars()
            .enumerate()
            .filter(|(idx, _)| *idx > closing_brace_idx)
            .map(|(_, c)| c)
            .collect();
        return (Some(remaining_list.trim_matches(',').to_string()), PacketValue::List(popped_list))
    }

    let tmp = list.split_once(',');
    match tmp {
        Some((popped_integer, remaining)) => {
            let parsed_integer = parse_integer(popped_integer);
            (Some(remaining.to_string()), parsed_integer)
        },
        None => {
            let parsed_integer = parse_integer(list.as_str());
            (None, parsed_integer)
        }
    }
}

// If there's an integer to parse, returns an Integer. If there's no string at
// all, returns a None. If there's some string, but not parseable to usize,
// panics.
fn parse_integer(s: &str) -> PacketValue {
    if s.is_empty() {
        return PacketValue::None
    }
    let parsed_integer = s.parse::<usize>().expect("Not reading an integer...");
    PacketValue::Integer(parsed_integer)
}

/// Height map
///
/// S = current position (elevation `a`)
/// E = target position (elevation `z`)
///
/// Elevations `a`-`z` (low to high).
///
/// Motion is orthogonal, no more than one step up in elevation (e.g. a->b,
/// e->a, ...)
#[derive(Clone)]
pub struct MapNode {
    pub x: isize,
    pub y: isize,
    pub elevation: isize,
    pub distance_from_initial: usize,
    pub is_visited: bool,
}

impl MapNode {
    pub fn new(x: isize, y: isize, elevation: isize, is_initial: bool) -> Self {
        Self {
            x,
            y,
            elevation,
            distance_from_initial: if is_initial { 0 } else { usize::MAX },  // Djikstra step 2.
            is_visited: false,  // Djikstra step 1.
        }
    }

    pub fn is_upward_neighbor(&self, other: &Self) -> bool {
        (((self.x - other.x).abs() == 1 && self.y == other.y)
        || ((self.y - other.y).abs() ==1 && self.x == other.x))
        && (other.elevation - self.elevation) <= 1
    }

    pub fn is_downward_neighbor(&self, other: &Self) -> bool {
        (((self.x - other.x).abs() == 1 && self.y == other.y)
        || ((self.y - other.y).abs() ==1 && self.x == other.x))
        && (self.elevation - other.elevation) <= 1
    }
}

impl Ord for MapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance_from_initial.cmp(&other.distance_from_initial)
    }
}

impl PartialOrd for MapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MapNode {
    fn eq(&self, other: &Self) -> bool {
        self.distance_from_initial == other.distance_from_initial
    }
}

impl Eq for MapNode {}

/// Return the node with the minimum estimated distance to initial.
pub fn get_next_node_coords(nodes: &HashMap<[isize; 2], MapNode>) -> Option<[isize; 2]>{
    nodes.values().filter(|&node| !node.is_visited).min().map(|node| [node.x, node.y])
}

/// Djikstra's algorithm.
///
/// Nodes have already been marked visited or nots

/// Going to try Djikstra's algorithm
///
///

/// CRT for the comms device
///
/// Each clock cycle, draws a single pixel on a 40x6 display. hi/lo is
/// determined by the location of a 3-wide sprite (CPU x +/- 1)
pub struct CRT {
    display: Vec<bool>,
    width: usize,
    height: usize,
}

impl CRT {
    pub fn new() -> Self {
        let width = 40;
        let height = 6;
        Self {
            display: vec![false; width*height],
            width,
            height,
        }
    }

    /// Run once per cycle. Draws on a lag (cycle 1 draws pixel 0).
    pub fn lazer_beam_it(&mut self, cycle: usize, sprite_pos: isize) {
        if cycle >= self.width * self.height {
            return
        }
        let crt_pos = (cycle-1) % 40;
        self.display[cycle-1] = (crt_pos as isize - sprite_pos).abs() <= 1;
    }

    pub fn visualize(&self) {
        for row in &self.display.iter().chunks(self.width) {
            for &pixel in row {
                if pixel {
                    print!("#");
                    continue
                }
                print!(".")
            }
            println!()
        }
    }
}

impl Default for CRT {
    fn default() -> Self {
        Self::new()
    }
}

/// CPU for the comms device
///
/// noop : 1 cycle
/// addx : 2 cycles (value changes *after* cycle ends)
pub struct CPU {
    pub x: isize,
    pub cycle: usize,
    queue: VecDeque<isize>,  // Stores delta x for upcoming clock cycles.
}

impl CPU {
    pub fn new() -> Self {
        Self {
            x: 1,
            // zeroth cycle is a no-op.
            cycle: 0,
            queue: VecDeque::from([0]),
        }
    }

    /// Queue up a no-op.
    pub fn noop(&mut self) {
        self.queue.push_back(0);
    }

    /// Queue up an add operation.
    pub fn addx(&mut self, dx: isize) {
        self.queue.push_back(0);
        self.queue.push_back(dx);
    }

    pub fn get_runtime_in_cycles(&self) -> usize {
        self.queue.len()
    }

    /// Executes a clock cycle if there's an instruction ready.
    ///
    /// Returns the current cycle, either as an Ok or an Err.
    pub fn execute_clock_cycle(&mut self) -> Result<usize, usize>{
        match self.queue.pop_front() {
            Some(dx) => {
                self.x += dx;
                self.cycle += 1;
                Ok(self.cycle)
            },
            None => {
                Err(self.cycle)
            }
        }
    }

    pub fn get_signal_strength(&self) -> isize {
        self.cycle as isize * self.x
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }


}

/// Rope end structure to track position on a grid.
///
/// Initial position is 0, 0.
pub struct RopeEnd {
    pub x: isize,
    pub y: isize,
    pub visited_spaces: HashSet<[isize; 2]>,  // Keep track of where it's been.
}

impl RopeEnd {
    /// Create a new rope end at the starting position.
    pub fn new() -> Self {
        let mut visited_spaces: HashSet<[isize; 2]> = HashSet::new();
        visited_spaces.insert([0, 0]);
        Self {
            x: 0,
            y: 0,
            visited_spaces,
        }
    }

    /// Make a delta move in x and y.
    ///
    /// e.g. move_delta(1, 1) would increase x and y each by 1.
    pub fn move_delta(&mut self, deltas: [isize; 2]) {
        self.x += deltas[0];
        self.y += deltas[1];
        self.visited_spaces.insert([self.x, self.y]);
    }

    pub fn follow(&mut self, leader: &Self) {
        let dx = leader.x - self.x ;
        let dy = leader.y - self.y ;
        if (dx.abs() > 1) || (dy.abs() > 1) {
            // Only ever move one space in each direction.
            self.move_delta([dx.signum(), dy.signum()])
        }
    }
}

impl Default for RopeEnd {
    fn default() -> Self {
        Self::new()
    }
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
        let mut parents: Vec<String> = Vec::new();
        // Be explicit about the location of a folder to protect against repeat
        // names in different parent directories.
        let mut parent: String = cwd.to_owned();
        let filename = parent.to_owned() + name;
        loop {
            let offset = parent.rfind('/').unwrap();
            parent.truncate(offset);
            if parent.trim().is_empty() {
                break
            }
            parents.push(parent.to_owned());
        }
        file_sizes.insert(filename.to_owned(), size);
        file_owners.insert(filename.to_owned(), parents);
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
