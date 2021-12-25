use std::{fs, path::PathBuf};

fn load_input_file(filename: &str) -> String {
    fs::read_to_string(PathBuf::from(filename)).expect("Could not read file")
}

fn parse_input(file: String) -> Vec<usize> {
    file.split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

#[derive(Debug, Clone)]
struct State {
    fish_by_age: Vec<u128>,
}

fn simulate(initial_state: State, days: u32) -> State {
    let mut state = initial_state;
    let mut d = days;
    while d > 0 {
        let mut new_state = State {
            fish_by_age: vec![0; 9],
        };
        d -= 1;
        for (age, count) in state.fish_by_age.iter().enumerate() {
            // println!("age={}, count={}", age, count);
            if age > 0 {
                new_state.fish_by_age[age - 1] += count;
            } else {
                // 0-fish become 6
                new_state.fish_by_age[6] += count;
                // same number of new fish born at 8
                new_state.fish_by_age[8] += count;
            }
        }

        state = new_state;
    }

    state
}

pub fn solve_1() -> u128 {
    let file = load_input_file("src/input/day6.txt");
    let input = parse_input(file);

    let mut initial_state = State {
        fish_by_age: vec![0; 9],
    };
    for fish_age in input {
        initial_state.fish_by_age[fish_age] += 1;
    }

    let final_state = simulate(initial_state, 80);

    let mut sum = 0;
    for count in final_state.fish_by_age {
        sum += count;
    }
    sum
}

pub fn solve_2() -> u128 {
    let file = load_input_file("src/input/day6.txt");
    let input = parse_input(file);

    let mut initial_state = State {
        fish_by_age: vec![0; 9],
    };
    for fish_age in input {
        initial_state.fish_by_age[fish_age] += 1;
    }

    let final_state = simulate(initial_state, 256);

    let mut sum = 0;
    for count in final_state.fish_by_age {
        sum += count;
    }
    sum
}
