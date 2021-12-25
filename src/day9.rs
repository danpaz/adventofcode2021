use std::{fs, path::PathBuf};

fn parse_input(filename: &str) -> Map {
    let cells = fs::read_to_string(PathBuf::from(filename))
        .expect("Could not read file")
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| {
                    return Cell {
                        value: x.to_digit(10).unwrap(),
                        is_basin: false,
                    };
                })
                .collect()
        })
        .collect();
    Map { cells: cells }
}

struct Map {
    cells: Vec<Vec<Cell>>,
}

struct Cell {
    value: u32,
    is_basin: bool,
}

fn is_low_point(map: &Vec<Vec<Cell>>, i: usize, j: usize) -> bool {
    let max_i = map.len() - 1;
    let max_j = map[0].len() - 1;
    let num = map[i][j].value;
    if i > 0 {
        let top = map[i - 1][j].value;
        if num >= top {
            return false;
        }
    }
    if i < max_i {
        let bottom = map[i + 1][j].value;
        if num >= bottom {
            return false;
        }
    }
    if j > 0 {
        let left = map[i][j - 1].value;
        if num >= left {
            return false;
        }
    }
    if j < max_j {
        let right = map[i][j + 1].value;
        if num >= right {
            return false;
        }
    }

    true
}

fn count_higher_neighbors(map: &mut Vec<Vec<Cell>>, i: usize, j: usize) -> u32 {
    let max_i = map.len() - 1;
    let max_j = map[0].len() - 1;
    let num = map[i][j].value;

    let mut sum = 1;
    if i > 0 {
        let top = map[i - 1][j].value;
        if num < top && top != 9 && !map[i - 1][j].is_basin {
            map[i - 1][j].is_basin = true;
            sum += count_higher_neighbors(map, i - 1, j);
        }
    }
    if i < max_i {
        let bottom = map[i + 1][j].value;
        if num < bottom && bottom != 9 && !map[i + 1][j].is_basin {
            map[i + 1][j].is_basin = true;
            sum += count_higher_neighbors(map, i + 1, j);
        }
    }
    if j > 0 {
        let left = map[i][j - 1].value;
        if num < left && left != 9 && !map[i][j - 1].is_basin {
            map[i][j - 1].is_basin = true;
            sum += count_higher_neighbors(map, i, j - 1);
        }
    }
    if j < max_j {
        let right = map[i][j + 1].value;
        if num < right && right != 9 && !map[i][j + 1].is_basin {
            map[i][j + 1].is_basin = true;
            sum += count_higher_neighbors(map, i, j + 1);
        }
    }

    sum
}

pub fn solve_1() -> u32 {
    let map = parse_input("src/input/day9.txt");

    let num_rows = map.cells.len();
    let num_cols = map.cells[0].len();

    let mut sum = 0;

    for i in 0..num_rows {
        for j in 0..num_cols {
            if is_low_point(&map.cells, i, j) {
                sum += 1 + map.cells[i][j].value;
            }
        }
    }
    sum
}

pub fn solve_2() -> u32 {
    let mut map = parse_input("src/input/day9.txt");

    let num_rows = map.cells.len();
    let num_cols = map.cells[0].len();

    let mut basins: Vec<u32> = vec![];

    for i in 0..num_rows {
        for j in 0..num_cols {
            if is_low_point(&map.cells, i, j) {
                basins.push(count_higher_neighbors(&mut map.cells, i, j));
            }
        }
    }

    basins.sort_by(|a, b| b.cmp(a));

    basins[0] * basins[1] * basins[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_basin() {
        let map: Vec<Vec<u32>> = vec![
            vec![9, 9, 9, 9, 9],
            vec![9, 5, 4, 5, 9],
            vec![9, 4, 3, 4, 9],
            vec![9, 5, 4, 5, 9],
            vec![9, 9, 9, 9, 9],
        ];

        let mut cells = map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|n| Cell {
                        value: *n,
                        is_basin: false,
                    })
                    .collect()
            })
            .collect();

        assert_eq!(count_higher_neighbors(&mut cells, 2, 2), 9);
    }
}
