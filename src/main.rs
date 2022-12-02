#![allow(dead_code)]

use aoc2022::{read_file, rps_explicit, rps_implicit};

fn main() {
    day2();
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
