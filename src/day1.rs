use std::{fs, path::PathBuf};

fn load_input_file(filename: &str) -> String {
    fs::read_to_string(PathBuf::from(filename)).expect("Could not read file")
}

fn count_increases(nums: Vec<u32>) -> u32 {
    let mut increases: u32 = 0;
    let mut prev = std::u32::MAX;

    for num in nums {
        if num > prev {
            increases += 1;
        }
        prev = num;
    }

    increases
}
 
pub fn solve_1() -> u32{
    let input = load_input_file("src/input/day1.txt").lines().map(|x| x.parse::<u32>().unwrap()).collect();
    count_increases(input)
}

pub fn solve_2() -> u32 {
    let input : Vec<u32> = load_input_file("src/input/day1.txt").lines().map(|x| x.parse::<u32>().unwrap()).collect();

    let input_len = input.len();
    let mut windowsums : Vec<u32> = vec![];
    let window_size : usize = 3;

    for i in 0..input_len {
        if (i as usize + window_size) > input_len {
            break;
        }
        let mut windowsum = 0;
        for j in i..(i+window_size) {
            windowsum += input[(j as usize)];
        }
        windowsums.push(windowsum);
    }

    count_increases(windowsums)
}