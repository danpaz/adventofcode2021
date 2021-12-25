use std::{
    fmt::{self},
    fs,
    path::PathBuf,
};

fn parse_input(filename: &str) -> Vec<Vec<Octopus>> {
    fs::read_to_string(PathBuf::from(filename))
        .expect("Could not read file")
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| Octopus {
                    level: x.to_digit(10).unwrap(),
                    flashing: false,
                })
                .collect()
        })
        .collect()
}

struct Octopus {
    level: u32,
    flashing: bool,
}

impl fmt::Debug for Octopus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.flashing {
            write!(f, ">{}<", &self.level)
        } else {
            write!(f, " {} ", &self.level)
        }
    }
}

fn flash_neighbors(map: &mut Vec<Vec<Octopus>>, i: usize, j: usize) {
    // println!("FLASH ({}, {})", i, j);
    let mut rows: Vec<usize> = vec![];

    if i > 0 {
        rows.push(i - 1);
    }
    rows.push(i);
    if i < map.len() - 1 {
        rows.push(i + 1);
    }

    let mut cols: Vec<usize> = vec![];
    if j > 0 {
        cols.push(j - 1);
    }
    cols.push(j);
    if j < map[0].len() - 1 {
        cols.push(j + 1);
    }

    // println!("rows={:?}", rows);
    // println!("cols={:?}", cols);
    // println!("OK");
    for r in rows {
        for &c in &cols {
            if r == i && c == j {
                continue;
            }
            // println!("({}, {})", r, c);
            let mut o = &mut map[r][c];
            o.level += 1;
            if !o.flashing && o.level > 9 {
                o.flashing = true;
                flash_neighbors(map, r, c);
            }
        }
    }
}

fn simulate_step(map: &mut Vec<Vec<Octopus>>) -> u32 {
    let mut num_flashes = 0;

    // First, the energy level of each octopus increases by 1.
    for row in map.iter_mut() {
        for o in row.iter_mut() {
            o.level += 1;
        }
    }
    // for line in map.iter() {
    //     println!("{:?}", line);
    // }
    // println!("");

    // Then, any octopus with an energy level greater than 9 flashes.
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let o = &mut map[i][j];
            if !o.flashing && o.level > 9 {
                o.flashing = true;
                flash_neighbors(map, i, j);
            }
        }
    }
    // for line in map.iter() {
    //     println!("{:?}", line);
    // }
    // println!("");
    // Reset all flashing octopuses to 0
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j].flashing && map[i][j].level > 9 {
                num_flashes += 1;
                map[i][j].flashing = false;
                map[i][j].level = 0;
            }
        }
    }
    num_flashes
}

pub fn solve_1() -> u32 {
    let mut input = parse_input("src/input/day11.txt");
    let mut num_flashes: u32 = 0;

    for _ in 1..=100 {
        num_flashes += simulate_step(&mut input);
    }

    num_flashes
}

pub fn solve_2() -> u32 {
    let mut input = parse_input("src/input/day11.txt");

    let target: u32 = (input.len() * input[0].len()) as u32;
    let mut i = 0;
    loop {
        i += 1;
        let num_flashes = simulate_step(&mut input);
        // println!("After step {}", i);
        // for line in input.iter() {
        //     println!("{:?}", line);
        // }
        // println!("");

        if num_flashes == target {
            return i;
        }
    }

    // num_flashes
}
