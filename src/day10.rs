use std::{collections::HashMap, fs, path::PathBuf};

fn parse_input(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(PathBuf::from(filename))
        .expect("Could not read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn solve_1() -> u32 {
    let input = parse_input("src/input/day10.txt");
    let pairs: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let scores: HashMap<char, u32> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let mut sum: u32 = 0;

    for line in &input {
        let mut stack: Vec<char> = vec![];
        for char in line {
            if pairs.contains_key(char) {
                stack.push(*char);
            } else if pairs.get(&stack.pop().unwrap()).unwrap() != char {
                sum += scores.get(&char).unwrap();
                break;
            }
        }
    }

    sum
}

pub fn solve_2() -> u128 {
    let input = parse_input("src/input/day10.txt");
    let pairs: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    let points: HashMap<char, u128> = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let mut stacks: Vec<Vec<char>> = vec![];
    for line in &input {
        let stack: Vec<char> = vec![];
        stacks.push(stack);
        for char in line {
            if pairs.contains_key(char) {
                stacks.last_mut().unwrap().push(*char);
            } else if pairs
                .get(&stacks.last_mut().unwrap().pop().unwrap())
                .unwrap()
                != char
            {
                stacks.pop();
                break;
            }
        }
    }

    let mut scores: Vec<u128> = vec![];
    for mut stack in stacks {
        let mut score: u128 = 0;
        while let Some(i) = stack.pop() {
            score = score * 5;
            score += points.get(&i).unwrap();
        }
        scores.push(score);
    }

    scores.sort();
    scores[(scores.len() as f32 / 2 as f32).floor() as usize]
}
