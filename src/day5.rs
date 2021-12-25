use std::{fs, path::PathBuf};

fn load_input_file(filename: &str) -> String {
    fs::read_to_string(PathBuf::from(filename)).expect("Could not read file")
}

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Line {
    fn is_straight(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }
}

#[derive(Debug)]
struct Map {
    points: Vec<Vec<usize>>,
}

fn parse_input(file: String, filter_diagonals: bool) -> (Vec<Line>, usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;
    let lines = file
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();
            let (x_1, y_1) = from.split_once(",").unwrap();
            let (x_2, y_2) = to.split_once(",").unwrap();

            let x1 = x_1.parse::<usize>().unwrap();
            let y1 = y_1.parse::<usize>().unwrap();
            let x2 = x_2.parse::<usize>().unwrap();
            let y2 = y_2.parse::<usize>().unwrap();

            max_x = std::cmp::max(max_x, x1);
            max_x = std::cmp::max(max_x, x2);
            max_y = std::cmp::max(max_y, y1);
            max_y = std::cmp::max(max_y, y2);
            Line { x1, y1, x2, y2 }
        })
        .filter(|line| line.is_straight() || !(filter_diagonals && !line.is_straight()))
        .collect();

    (lines, max_x, max_y)
}

fn get_xy_pairs(line: &Line) -> Vec<(usize, usize)> {
    let mut pairs: Vec<(usize, usize)> = vec![];
    if line.is_straight() {
        let x_range = std::cmp::min(line.x1, line.x2)..=std::cmp::max(line.x1, line.x2);
        let y_range = std::cmp::min(line.y1, line.y2)..=std::cmp::max(line.y1, line.y2);
        for x in x_range {
            for y in y_range.clone() {
                pairs.push((x, y));
            }
        }
    } else {
        // Diagonal case, use y = mx + c
        let slope: i32 = (line.y2 as i32 - line.y1 as i32) / (line.x2 as i32 - line.x1 as i32);
        let intercept: i32 = line.y1 as i32 - slope * line.x1 as i32;

        // Have to box it to contain either Range or Rev<Range>
        let mut x_range: Box<dyn Iterator<Item = usize>> = Box::new(line.x1..=line.x2);
        if line.x1 > line.x2 {
            x_range = Box::new((line.x2..=line.x1).rev());
        }

        for x in x_range {
            let y = slope * x as i32 + intercept;
            pairs.push((x, y as usize));
        }
    }
    pairs
}

pub fn solve_1() -> u32 {
    let file = load_input_file("src/input/day5.txt");
    let (lines, max_x, max_y) = parse_input(file, true);

    let mut map = Map {
        points: vec![vec![0; max_y + 1]; max_x + 1],
    };
    let mut count: u32 = 0;

    for line in lines {
        for (x, y) in get_xy_pairs(&line) {
            map.points[x][y] += 1;
            if map.points[x][y] == 2 {
                // Increment count of points that have reached 2
                count += 1;
            }
        }
    }

    count
}

pub fn solve_2() -> u32 {
    let file = load_input_file("src/input/day5.txt");
    let (lines, max_x, max_y) = parse_input(file, false);

    let mut map = Map {
        points: vec![vec![0; max_y + 1]; max_x + 1],
    };
    let mut count: u32 = 0;

    for line in lines {
        for (x, y) in get_xy_pairs(&line) {
            map.points[x][y] += 1;
            if map.points[x][y] == 2 {
                // Increment count of points that have reached 2
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_diagonals() {
        let lines: Vec<Line> = vec![
            Line {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 1,
            }, // straight
            Line {
                x1: 0,
                y1: 0,
                x2: 1,
                y2: 1,
            }, // diagonal
        ];
        let mut filter_diagonals = true;
        let length_filtered = lines
            .iter()
            .filter(|&line| line.is_straight() || !(filter_diagonals && !line.is_straight()))
            .collect::<Vec<&Line>>()
            .len();

        filter_diagonals = false;
        let length_unfiltered = lines
            .iter()
            .filter(|&line| line.is_straight() || !(filter_diagonals && !line.is_straight()))
            .collect::<Vec<&Line>>()
            .len();

        assert_eq!(length_filtered, 1);
        assert_eq!(length_unfiltered, 2);
    }

    #[test]
    fn get_xy_pairs_straight_line() {
        let line = Line {
            x1: 0,
            y1: 0,
            x2: 2,
            y2: 0,
        };

        assert_eq!(get_xy_pairs(&line), vec![(0, 0), (1, 0), (2, 0)]);
    }

    #[test]
    fn get_xy_pairs_diagonal_line() {
        let mut line = Line {
            x1: 1,
            y1: 1,
            x2: 3,
            y2: 3,
        };

        assert_eq!(get_xy_pairs(&line), vec![(1, 1), (2, 2), (3, 3)]);

        line = Line {
            x1: 9,
            y1: 7,
            x2: 7,
            y2: 9,
        };

        assert_eq!(get_xy_pairs(&line), vec![(9, 7), (8, 8), (7, 9)]);
    }
}
