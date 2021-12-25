use std::{fs, path::PathBuf};
use std::str::FromStr;

fn load_input_file(filename: &str) -> String {
    fs::read_to_string(PathBuf::from(filename)).expect("Could not read file")
}

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    amount: i32,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

pub fn solve_1() -> i32 { 
    let input = load_input_file("src/input/day2.txt");
    let commands = input.lines()
        .map(|line| {
            let mut split = line.split(" ");
            let (dir, num) = (split.next().unwrap(), split.next().unwrap().parse::<i32>().unwrap());
            Command {
                direction: Direction::from_str(dir).unwrap(),
                amount: num,
            }
        });

    let mut pos = Position { x: 0, y: 0, aim: 0 };

    for command in commands {
        match command.direction {
            Direction::Forward => { pos.x += command.amount; },
            Direction::Up => { pos.y -= command.amount; },
            Direction::Down => { pos.y += command.amount; },
        }
    }
    pos.x * pos.y
}


pub fn solve_2() -> i32 { 
    let input = load_input_file("src/input/day2.txt");
    let commands = input.lines()
        .map(|line| {
            let mut split = line.split(" ");
            let (dir, num) = (split.next().unwrap(), split.next().unwrap().parse::<i32>().unwrap());
            Command {
                direction: Direction::from_str(dir).unwrap(),
                amount: num,
            }
        });

    let mut pos = Position { x: 0, y: 0, aim: 0 };

    for command in commands {
        match command.direction {
            Direction::Forward => { 
                pos.x += command.amount; 
                pos.y += pos.aim * command.amount;
            },
            Direction::Up => { 
                pos.aim -= command.amount; 
            },
            Direction::Down => { 
                pos.aim += command.amount; 
            },
        }
    }
    pos.x * pos.y
}