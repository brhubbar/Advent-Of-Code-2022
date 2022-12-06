#![allow(dead_code)]
use aoc2022::{
    read_file,
    find_marker,
    create_stack_regex,
    make_moves,
    is_full_overlap,
    is_partial_overlap,
    find_missort,
    prioritize_items,
    identify_badge,
    rps_explicit,
    rps_implicit,
};

fn main() {
    day6();
}

/// Packet detection
fn day6() {
    let contents = read_file("data/day6.txt");
    let packet_flag: usize = find_marker(contents.as_str(), 4);
    println!("Day 6, Part 1: {}", packet_flag);
    let message_flag: usize = find_marker(contents.as_str(), 14);
    println!("Day 6, Part 2: {}", message_flag);
}

/// FIgure out where the crates are going to be.
fn day5() {
    let mut crates = read_file("data/day5_order.txt");
    crates = crates.trim_end().to_string();
    let mut moves = read_file("data/day5_moves.txt");
    moves = moves.trim_end().to_string();

    let finders = create_stack_regex(crates.split('\n').last().expect("Shit"));

    let mut stacks: Vec<Vec<&str>> = Vec::new();
    for finder in finders {
        let mut stack: Vec<&str> = Vec::new();

        for stack_captures in finder.captures_iter(crates.as_str()) {
            stack.push(stack_captures.get(1).unwrap().as_str());
        }
        // Top of the stack needs to be the end of the Vec.
        stack.reverse();
        stacks.push(stack);
    }
    for move_set in moves.split('\n') {
        make_moves(move_set, &mut stacks)
    }
    print!("Day 5, Part 1: ");
    for stack in stacks {
        print!("{}", stack[stack.len()-1]);
    }


}

/// Check for overlapped assignments between paired elves.
fn day4() {
    let contents = read_file("data/day4.txt");
    let mut fully_wasted = 0;
    let mut kinda_wasted = 0;
    for pair in contents.split('\n') {
        if pair.is_empty() {
            continue
        }
        if is_full_overlap(pair) { fully_wasted += 1; };
        if is_partial_overlap(pair) { kinda_wasted += 1; };

    }
    println!("Day 4, Part 1: {:?}", fully_wasted);
    println!("Day 4, Part 2: {:?}", kinda_wasted);
}

/// Check for mis-sorted and lost items stored in rucksacks.
fn day3() {
    let contents = read_file("data/day3.txt");
    let mut prioritays: u32 = 0;
    let mut missorted: u8;
    let rucksacks: Vec<&str> = contents.split('\n').collect();
    for rucksack in &rucksacks {
        if rucksack.is_empty() {
            continue
        }
        missorted = find_missort(rucksack);
        prioritays += u32::from(prioritize_items(missorted));
    }
    println!("Day 3, Part 1: {:?}", prioritays);

    let group_size = 3;
    let mut badge: u8;
    prioritays = 0;
    for group in rucksacks.iter().collect::<Vec<&&str>>().chunks(group_size) {
        if group.len() < group_size {
            // Made it to the end, I presume.
            continue
        }
        badge = identify_badge(group);
        prioritays += u32::from(prioritize_items(badge));
    }

    println!("Day 3, Part 2: {:?}", prioritays);
}

/// Compute the score of a rock paper scissors match
fn day2() {
    let contents = read_file("data/day2.txt");

    let mut sum_part_1: u32 = 0;
    let mut sum_part_2: u32 = 0;
    for row in contents.split('\n') {
        if row.is_empty() {
            continue
        }
        sum_part_1 += u32::from(rps_explicit(row));
        sum_part_2 += u32::from(rps_implicit(row));

    }
    println!("Day 2, Part 1: {:?}", sum_part_1);
    println!("Day 2, Part 2: {:?}", sum_part_2);
}

/// Compute calories carried by each of the elves and identify the top carriers.
fn day1() {
    let file_path = "data/day1.txt";
    let contents = read_file(file_path);

    let mut total_calories: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    for row in contents.split('\n') {
        if row.is_empty() {
            // Note that this copies sum rather than moves because i32
            // implements the Copy trait.
            total_calories.push(sum);
            // println!("{:?}", sum);
            sum = 0;
            continue
        }
        // source: https://stackoverflow.com/a/27683271
        // println!("{:?}", row);
        sum += row.parse::<i32>().unwrap();
    }
    total_calories.sort();
    total_calories.reverse();
    let max_calories = total_calories[0];
    println!("Day 1, Part 1: {:?}", max_calories);
    let top_three: i32 = total_calories[0..3].iter().sum();
    println!("Day 1, Part 2: {:?}", top_three);
}
