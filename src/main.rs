#![allow(dead_code)]
use std::collections::HashMap;
use array2d::Array2D;
use itertools::Itertools;

use aoc2022::{
    read_file,
    RopeEnd,
    parse_comms_comms,
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
    day9();
}

/// Calculate rope motion.
fn day9() {
    let moves = read_file("data/day9.txt");
    let mut head = RopeEnd::new();
    let mut tail = RopeEnd::new();
    let directions_map: HashMap<&str, [isize; 2]> = HashMap::from([
        ("U", [0, 1]),
        ("D", [0, -1]),
        ("L", [-1, 0]),
        ("R", [1, 0]),
    ]);

    for move_ in moves.split('\n') {
        if move_.trim().is_empty() {
            continue
        }
        let (direction, num_steps) = move_
            .split_once(' ')
            .expect("Move isn't formatted as expected");
        let move_coords = directions_map
            .get(direction)
            .expect("Unsupported move direction");
        for _ in 0..num_steps.parse().unwrap() {
            head.move_delta(*move_coords);
            // Figure out where tail needs to move..
            let dx = head.x - tail.x ;
            let dy = head.y - tail.y ;
            if (dx.abs() > 1) || (dy.abs() > 1) {
                // Only ever move one space in each direction.
                tail.move_delta([dx.signum(), dy.signum()])
            }
            println!("({}, {}), ({}, {})", head.x, head.y, tail.x, tail.y);
        }
    }

    let total_visits = tail.visited_spaces.len();
    println!("Day 9, Part 1: {total_visits}")
}

/// Plan a treehouse.
fn day8() {
    let contents = read_file("data/day8.txt");
    let mut grid_rows: Vec<Vec<i8>> = Vec::new();
    for row in contents.split('\n').map(str::trim) {
        if row.is_empty() {
            continue
        }
        let mut grid_row: Vec<i8> = Vec::with_capacity(row.len());
        for digit in row.chars() {
            let digit_str = digit.to_string();
            grid_row.push(digit_str.parse::<i8>().expect("Not a number???"));
        }
        grid_rows.push(grid_row);
    }
    let grid: Array2D<i8> = Array2D::from_rows(&grid_rows);

    let mut is_visible: Array2D<bool> = Array2D::filled_with(false, grid.num_rows(), grid.num_columns());
    let mut scenic_score: Array2D<usize> = Array2D::filled_with(0, grid.num_rows(), grid.num_columns());
    // Trees can only be hidden if not on the edge. Similarly, scenic score can
    // only be non-zero is not on the edge.
    for (row_idx, col_idx) in (1..grid.num_rows()-1).cartesian_product(1..grid.num_columns()-1) {
        let tree_height = grid[(row_idx, col_idx)];
        scenic_score[(row_idx, col_idx)] = 1;
        // println!("{row_idx}, {col_idx}, {tree_height}");

        let mut row = grid.row_iter(row_idx);
        // Take the part of the row after col_idx
        let taller = row.enumerate()
            .filter(|&(idx, h)| idx>col_idx && *h>=tree_height)
            .map(|(idx, _)| idx)
            .next();
        match taller {
            Some(idx) => {
                // println!("{col_idx} --> {idx}");
                scenic_score[(row_idx, col_idx)] *= idx - col_idx;
            },
            None => {
                let end = grid.num_columns() - 1 - col_idx;
                // println!("{col_idx} --> {end}");
                is_visible[(row_idx, col_idx)] = true;
                scenic_score[(row_idx, col_idx)] *= end;
            },
        }
        row = grid.row_iter(row_idx);
        let taller = row.enumerate()
            .filter(|&(idx, h)| idx<col_idx && *h>=tree_height)
            .map(|(idx, _)| idx)
            .last();
        match taller {
            Some(idx) => {
                scenic_score[(row_idx, col_idx)] *= col_idx - idx;
            },
            None => {
                is_visible[(row_idx, col_idx)] = true;
                scenic_score[(row_idx, col_idx)] *= col_idx;
            },
        }

        let mut col = grid.column_iter(col_idx);
        // Take the part of the row after row
        let taller = col.enumerate()
            .filter(|&(idx, h)| idx>row_idx && *h>=tree_height)
            .map(|(idx, _)| idx)
            .next();
        match taller {
            Some(idx) => {
                scenic_score[(row_idx, col_idx)] *= idx - row_idx;
            },
            None => {
                is_visible[(row_idx, col_idx)] = true;
                scenic_score[(row_idx, col_idx)] *= grid.num_rows() - 1 - row_idx;
            },
        }
        col = grid.column_iter(col_idx);
        let taller = col.enumerate()
            .filter(|&(idx, h)| idx<row_idx && *h>=tree_height)
            .map(|(idx, _)| idx)
            .last();
        match taller {
            Some(idx) => {
                scenic_score[(row_idx, col_idx)] *= row_idx - idx;
            },
            None => {
                is_visible[(row_idx, col_idx)] = true;
                scenic_score[(row_idx, col_idx)] *= row_idx;
            },
        }
    }

    let n_visible = is_visible.elements_row_major_iter().filter(|&b| *b).count()
        + grid.num_columns()*2 + grid.num_rows()*2 - 4;  // Double counted the corners
    println!("{is_visible:?}");
    println!("Day 8, Part 1: {n_visible}");

    let best_score = scenic_score.elements_row_major_iter().max().unwrap();
    println!("Day 8, Part 2: {best_score}");

}

/// Find big files.
fn day7() {
    let contents = read_file("data/day7.txt");
    let mut cwd: String = "".to_owned();
    let mut file_owners: HashMap<String, Vec<String>> = HashMap::new();
    let mut file_sizes: HashMap<String, usize> = HashMap::new();
    // Parse the commands and build out the tree.
    for cmd in contents.split('$').map(str::trim) {
        if cmd.is_empty() {
            continue
        }
        cwd = parse_comms_comms(cwd, cmd, &mut file_owners, &mut file_sizes);
    }

    println!("{:?}", file_owners);
    println!("{:?}", file_sizes);

    // Aggregate size of each directory.
    let mut folder_sizes: HashMap<String, usize> = HashMap::new();
    for file_name in file_owners.keys() {
        let file_size = file_sizes.get(file_name).unwrap();
        *folder_sizes.entry("/".to_string()).or_insert(0) += *file_size;
        for folder in file_owners.get(file_name).unwrap() {
            *folder_sizes.entry(folder.to_owned()).or_insert(0) += *file_size;
        }
    }
    // find those under the cap size.
    let mut arbitrary_sum: usize = 0;
    for folder in folder_sizes.values() {
        if *folder <= 100000 {
            arbitrary_sum += *folder
        }
    }
    println!("{:?}", folder_sizes);
    println!("Day 7, Part 1: {}", arbitrary_sum);

    let total_space = 70000000;
    let needed_space = 30000000;
    // Find the smallest directory that, if deleted, would free up enough space
    // the file system.
    let minimum_delete_size = folder_sizes.get("/").unwrap() - (total_space - needed_space);
    let mut _smallest_possible_directory = "/";
    let mut planned_delete_size = *folder_sizes.get("/").unwrap();
    for (folder, size) in folder_sizes.iter() {
        if *size > minimum_delete_size && *size < planned_delete_size {
            planned_delete_size = *size;
            _smallest_possible_directory = folder;
        }
    }
    println!("Day 7, Part 2: {}", planned_delete_size)
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
