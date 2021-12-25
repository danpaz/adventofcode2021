use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

#[derive(Debug)]
struct Node {
    id: String,
    is_small: bool,
}
type Graph = HashMap<String, Vec<Node>>;

#[derive(Debug, Clone)]
struct PathToEnd {
    ids: Vec<String>,
    visited_small: HashSet<String>,
    has_visited_small_twice: bool,
}

fn is_lowercase(s: &str) -> bool {
    s == s.to_lowercase()
}

fn parse_input(filename: &str) -> Graph {
    let mut input: Graph = HashMap::new();
    fs::read_to_string(PathBuf::from(filename))
        .expect("Could not read file")
        .lines()
        .for_each(|line| {
            let (from, to) = line.split_once("-").unwrap();

            // from -> to
            if let Some(neighbors) = input.get_mut(from) {
                neighbors.push(Node {
                    id: to.to_string(),
                    is_small: is_lowercase(to),
                });
            } else {
                input.insert(
                    from.to_string(),
                    vec![Node {
                        id: to.to_string(),
                        is_small: is_lowercase(to),
                    }],
                );
            }

            // to -> from
            if let Some(neighbors) = input.get_mut(to) {
                neighbors.push(Node {
                    id: from.to_string(),
                    is_small: is_lowercase(from),
                });
            } else {
                input.insert(
                    to.to_string(),
                    vec![Node {
                        id: from.to_string(),
                        is_small: is_lowercase(from),
                    }],
                );
            }
        });
    input
}

fn build_paths(
    paths: &mut Vec<PathToEnd>,
    graph: &Graph,
    path: &PathToEnd,
    from: &String,
    allow_twice: bool,
) {
    if let Some(neighbors) = graph.get(from) {
        for to in neighbors {
            let mut new_path = path.clone();
            new_path.ids.push(from.clone());
            if to.id == "end" {
                // Found a path!
                // println!("{} => {} DONE", from, to.id);

                new_path.ids.push(to.id.clone());
                paths.push(new_path);
            } else if to.is_small && path.visited_small.contains(&to.id) {
                if allow_twice
                    && !new_path.has_visited_small_twice
                    && to.id != "start"
                    && to.id != "end"
                {
                    new_path.has_visited_small_twice = true;

                    // println!("{} => {} CONTINUE", from, to.id);
                    // continue branched path

                    if to.is_small {
                        new_path.visited_small.insert(to.id.clone());
                    }
                    // new_path.ids.push(to.id.clone());
                    build_paths(paths, graph, &new_path, &to.id, allow_twice);
                } else {
                    // println!("{} => {} TOO SMALL", from, to.id);
                    // deadend
                    continue;
                }
            } else {
                // println!("{} => {} CONTINUE", from, to.id);
                // continue branched path

                if to.is_small {
                    new_path.visited_small.insert(to.id.clone());
                }
                // new_path.ids.push(to.id.clone());
                build_paths(paths, graph, &new_path, &to.id, allow_twice);
            }
        }
    } else {
        // println!("{} => DEADEND", from);
    }
}

pub fn solve_1() -> usize {
    let input = parse_input("src/input/day12.txt");

    let mut paths: Vec<PathToEnd> = vec![];
    let mut visited_small = HashSet::new();
    visited_small.insert("start".to_string());
    let initial_path = PathToEnd {
        ids: vec![],
        visited_small: visited_small,
        has_visited_small_twice: false,
    };

    build_paths(
        &mut paths,
        &input,
        &initial_path,
        &"start".to_string(),
        false,
    );

    for p in &paths {
        println!("{:?}", p.ids);
    }

    paths.len()
}

pub fn solve_2() -> usize {
    let input = parse_input("src/input/day12.txt");

    let mut paths: Vec<PathToEnd> = vec![];
    let mut visited_small = HashSet::new();
    visited_small.insert("start".to_string());
    let initial_path = PathToEnd {
        ids: vec![],
        visited_small: visited_small,
        has_visited_small_twice: false,
    };

    build_paths(
        &mut paths,
        &input,
        &initial_path,
        &"start".to_string(),
        true,
    );

    for p in &paths {
        println!("{:?}", p.ids);
    }

    paths.len()
}
