use std::{fs, path::PathBuf};

fn load_input_file(filename: &str) -> String {
    fs::read_to_string(PathBuf::from(filename)).expect("Could not read file")
}

fn parse_input(file: String) -> Vec<u32> {
    file.split(",").map(|x| x.parse::<u32>().unwrap()).collect()
}

fn calc_fuel_linear(positions: &Vec<u32>, target: u32) -> u32 {
    let mut total_fuel = 0;
    for &pos in positions {
        total_fuel += (target as i32 - pos as i32).abs();
    }
    total_fuel as u32
}

// https://en.wikipedia.org/wiki/Triangular_number
fn calc_fuel_triangular(positions: &Vec<u32>, target: u32) -> u32 {
    let mut total_fuel = 0;
    for &pos in positions {
        let diff = (target as i32 - pos as i32).abs();
        total_fuel += diff * (diff + 1) / 2;
    }
    total_fuel as u32
}

pub fn solve_1() -> u32 {
    let positions = parse_input(load_input_file("src/input/day7.txt"));
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let mut min_fuel: u32 = u32::MAX;
    for i in min..=max {
        min_fuel = std::cmp::min(min_fuel, calc_fuel_linear(&positions, i));
    }

    min_fuel
}

pub fn solve_2() -> u32 {
    let positions = parse_input(load_input_file("src/input/day7.txt"));
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let mut min_fuel: u32 = u32::MAX;
    for i in min..=max {
        min_fuel = std::cmp::min(min_fuel, calc_fuel_triangular(&positions, i));
    }

    min_fuel
}
