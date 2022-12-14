#![allow(dead_code)]
use std::{collections::HashMap, cmp::Ordering};
use array2d::Array2D;
use itertools::Itertools;

use aoc2022::{
    read_file,
    Cave,
    compare_lists,
    get_next_node_coords,
    MapNode,
    CRT,
    CPU,
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
    day14();
}

/// Sand tracking
fn day14() {
    let cave_scan = read_file("data/day14.txt");
    let mut cave = Cave::default();
    cave.read_scan(cave_scan.trim());

    let mut drops_in_the_bucket = 0;
    while cave.add_grain_of_sand() {
        drops_in_the_bucket += 1;
    }

    cave.print_cave();
    println!("Day 14, Part 1: {drops_in_the_bucket}");

    // Continue to part 2.
    cave.part = 2;
    while cave.add_grain_of_sand() {
        drops_in_the_bucket += 1;
    }
    // Skips the plugging of the hole.
    drops_in_the_bucket += 1;

    cave.print_cave();
    println!("Day 14, Part 2: {drops_in_the_bucket}");
}

/// Packet translation
fn day13() {
    let packets = read_file("data/day13.txt");
    let mut count: usize = 0;
    for (packet_idx, packet_pair) in packets.split("\n\n").enumerate() {
        let (left, right) = packet_pair
            .split_once('\n')
            .expect("Not a packet pair.");
        if compare_lists(left.to_string(), right.to_string()) == Ordering::Less {
            // Properly sorted.
            println!("\n{packet_idx}:\n{left}\n{right}");
            count += packet_idx + 1;  // Elves index from 1
        }
    }
    println!("Day 13, Part 1: {count}");

    let mut packet_vec: Vec<String> = packets
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    packet_vec.push("[[2]]".to_string());
    packet_vec.push("[[6]]".to_string());
    packet_vec.sort_by(|left, right| compare_lists(left.to_owned(), right.to_owned()));

    let mut divider_packet_spot = 1;
    for (packet_idx, packet) in packet_vec.iter().enumerate() {
        if packet == "[[2]]" || packet == "[[6]]" {
            divider_packet_spot *= 1 + packet_idx;
            println!("**{packet}");
            continue
        }
        println!("{packet}");
    }

    println!("Day 13, Part 2: {divider_packet_spot}");
}

/// Path planning
fn day12() {
    let height_map = read_file("data/day12.txt");
    let mut map: HashMap<[isize; 2], MapNode> = HashMap::new();

    let mut _destination_node: [isize; 2] = [0, 0];

    for (y, row) in height_map.split('\n').enumerate() {
        for (x, character) in row.chars().enumerate() {
            let coords = [x as isize, y as isize];
            let elevation: isize;
            if character == 'S' {
                elevation = 'a' as isize;
            } else if character == 'E' {
                _destination_node = coords;
                elevation = 'z' as isize;
            } else {
                elevation = character as isize;
            };
            // map.insert(coords, MapNode::new(x as isize, y as isize, elevation, character=='S'));
            map.insert(coords, MapNode::new(x as isize, y as isize, elevation, character=='E'));
        }
    }

    'djikstra: loop {
        let next_node_coords = get_next_node_coords(&map);

        match next_node_coords {
            Some(node_coords) => {
                // if node_coords == _destination_node {  // Optimization for when the destination is known
                //     break 'djikstra
                // }
                {
                    let current_node = map.get(&node_coords).expect("No node here...").clone();
                    if current_node.distance_from_initial == usize::MAX {
                        // We've hit the end of visitable or reasonable nodes.
                        break 'djikstra
                    }
                    // for neighbor in map.values_mut().filter(|node| current_node.is_upward_neighbor(node) && !node.is_visited) {
                    for neighbor in map.values_mut().filter(|node| current_node.is_downward_neighbor(node) && !node.is_visited) {
                        // println!("({}, {})", neighbor.x, neighbor.y);
                        let current_distance = current_node.distance_from_initial + 1;
                        if current_distance < neighbor.distance_from_initial {
                            neighbor.distance_from_initial = current_distance;
                        }
                    }
                }
                let current_node = map.get_mut(&node_coords).unwrap();
                current_node.is_visited = true;
            },
            None => break,
        }
    }
    // println!("Day 12, Part 1: {}", map.get(&_destination_node).unwrap().distance_from_initial);
    let part2 = map.values().filter(|node| node.elevation == 'a' as isize).min().unwrap();
    println!("Day 12, Part 2: ({}, {}): {}", part2.x, part2.y, part2.distance_from_initial);
}

/// Rebuild the video/cpu for the comms device.
fn day10() {
    let operations = read_file("data/day10.txt");
    let mut cpu = CPU::new();
    let mut crt = CRT::new();
    // Queue up all operations.
    for operation in operations.split('\n').map(|x| x.trim()) {
        if operation.trim().is_empty() {
            continue
        }
        if operation == "noop" {
            cpu.noop();
        } else if operation.starts_with("addx") {
            let dx = operation.split(' ').last().unwrap().parse::<isize>().unwrap();
            cpu.addx(dx);
        }
    }

    let cycles_of_mild_interest: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let mut signal_strength: isize = 0;
    let mut current_cycle: usize;
    'frankie: loop {
        match cpu.execute_clock_cycle() {
            Ok(cycle) => current_cycle = cycle,
            Err(_) => break 'frankie,
        }
        crt.lazer_beam_it(current_cycle, cpu.x);

        if cycles_of_mild_interest.contains(&current_cycle) {
            signal_strength += cpu.get_signal_strength()
        }
    }

    println!("Day 10, Part 1: {signal_strength}");
    crt.visualize();
}

/// Calculate rope motion.
fn day9() {
    let moves = read_file("data/day9.txt");
    let part = 2;
    let rope_length: usize =
        if part == 1 {
            2
        } else {
            10
        };
    let mut rope: Vec<RopeEnd> = Vec::new();
    for _ in 0..rope_length {
        rope.push(RopeEnd::new());
    }
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
            for planck_length_idx in 0..(rope.len()-1) {
                if planck_length_idx == 0 {
                    rope[planck_length_idx].move_delta(*move_coords);
                }
                // Get two mutable references by snagging a slice. Not totally
                // clear on how this
                let (head, tail) = if let [head, tail] = &mut rope[planck_length_idx..=planck_length_idx+1] {
                    Some((head, tail))
                } else {
                    None
                }.unwrap();
                tail.follow(head);
            }
            // println!("({}, {}), ({}, {})", head.x, head.y, tail.x, tail.y);
        }
    }

    let total_visits = rope[rope_length-1].visited_spaces.len();
    println!("Day 9, Part {part}: {total_visits}")
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
