//
//    0:      1:      2:      3:      4:
//   aaaa    ....    aaaa    aaaa    ....
//  b    c  .    c  .    c  .    c  b    c
//  b    c  .    c  .    c  .    c  b    c
//   ....    ....    dddd    dddd    dddd
//  e    f  .    f  e    .  .    f  .    f
//  e    f  .    f  e    .  .    f  .    f
//   gggg    ....    gggg    gggg    ....

//    5:      6:      7:      8:      9:
//   aaaa    aaaa    aaaa    aaaa    aaaa
//  b    .  b    .  .    c  b    c  b    c
//  b    .  b    .  .    c  b    c  b    c
//   dddd    dddd    ....    dddd    dddd
//  .    f  e    f  .    f  e    f  .    f
//  .    f  e    f  .    f  e    f  .    f
//   gggg    gggg    ....    gggg    gggg
//

use std::{fs, path::PathBuf};

fn load_input_file(filename: &str) -> String {
    fs::read_to_string(PathBuf::from(filename)).expect("Could not read file")
}

fn parse_input(file: String) -> Vec<Display> {
    file.lines()
        .map(|line| {
            let (patterns_str, outputs_str) = line.split_once(" | ").unwrap();
            Display {
                patterns: patterns_str.split(" ").map(|s| s.to_string()).collect(),
                outputs: outputs_str.split(" ").map(|s| s.to_string()).collect(),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Display {
    patterns: Vec<String>,
    outputs: Vec<String>,
}

pub fn solve_1() -> u32 {
    let input = parse_input(load_input_file("src/input/day8.txt"));

    let mut sum: u32 = 0;
    let unique_segment_counts = vec![2, 3, 4, 7];
    for display in input {
        for output in display.outputs {
            if unique_segment_counts.contains(&(output.len() as u32)) {
                sum += 1;
            }
        }
    }

    sum
}

fn non_overlapping_char(subset: &String, superset: &String) -> Option<char> {
    for char in superset.chars() {
        if !subset.contains(char) {
            return Some(char);
        }
    }
    None
}

fn decode(display: &mut Display) -> Vec<u32> {
    // sort ten patterns by string length
    display.patterns.sort_by(|a, b| a.len().cmp(&b.len()));

    // println!("{:?}", display.patterns);
    let one = &display.patterns[0];
    let seven = &display.patterns[1];
    let four = &display.patterns[2];
    let candidate_two_three_fives = vec![
        &display.patterns[3],
        &display.patterns[4],
        &display.patterns[5],
    ];
    let candidate_zero_six_nines = vec![
        &display.patterns[6],
        &display.patterns[7],
        &display.patterns[8],
    ];
    let eight = &display.patterns[9];

    let mut top_right: char = 'x';
    let mut zero: &String = &String::from("");
    let mut six: &String = &String::from("");
    let mut nine: &String = &String::from("");

    for candidate in candidate_zero_six_nines {
        if let Some(char) = non_overlapping_char(candidate, one) {
            top_right = char;
            six = candidate;
            // println!("six={}", six);
        } else if let Some(_char) = non_overlapping_char(candidate, four) {
            zero = candidate;
            // println!("zero={}", zero);
        } else {
            nine = candidate;
            // println!("nine={}", nine);
        }
    }
    let mut two: &String = &String::from("");
    let mut three: &String = &String::from("");
    let mut five: &String = &String::from("");
    for candidate in candidate_two_three_fives {
        if let Some(_char) = non_overlapping_char(candidate, one) {
            if candidate.contains(top_right) {
                two = candidate;
                // println!("two={}", two);
            } else {
                five = candidate;
                // println!("five={}", five);
            }
        } else {
            three = candidate;
            // println!("three={}", three);
        }
    }

    let mappings = vec![zero, one, two, three, four, five, six, seven, eight, nine];

    let mut decoded: Vec<u32> = vec![];
    for output in &display.outputs {
        for (i, candidate) in mappings.iter().enumerate() {
            if output.len() == candidate.len()
                && non_overlapping_char(&output, &candidate).is_none()
            {
                // println!("DEBUG! {}={}", output, i);
                decoded.push(i as u32);
            }
        }
    }
    decoded
}

pub fn solve_2() -> u32 {
    let mut input = parse_input(load_input_file("src/input/day8.txt"));

    let mut total_sum: u32 = 0;
    for display in input.iter_mut() {
        let mut val: u32 = 0;
        let decoded = decode(display);
        for (i, n) in decoded.iter().rev().enumerate() {
            val += n * 10_u32.pow(i as u32);
        }
        total_sum += val;
    }

    total_sum
}
