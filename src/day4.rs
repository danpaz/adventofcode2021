use std::{fs, path::PathBuf};

fn load_input_file(filename: &str) -> String {
    fs::read_to_string(PathBuf::from(filename)).expect("Could not read file")
}

#[derive(Debug)]
struct Board {
    cells: Vec<Vec<Cell>>,
    matched_cells_by_row: Vec<u32>,
    matched_cells_by_col: Vec<u32>,
    won: bool,
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    num: u32,
    marked: bool,
}

fn calc_winning_score(board: &Board, draw: u32) -> u32 {
    let mut sum_unmarked: u32 = 0;
    for row in board.cells.iter() {
        for cell in row.iter() {
            if !cell.marked {
                sum_unmarked += cell.num;
            }
        }
    }
    sum_unmarked * draw
}

fn parse_input(file: String) -> (Vec<u32>, Vec<Board>) {
    let mut lines_iter = file.lines();

    let draws: Vec<u32> = lines_iter
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // skip empty line
    lines_iter.next();

    let mut boards: Vec<Board> = vec![];
    let lines = lines_iter.collect::<Vec<&str>>();
    let mut i = 0;
    while i < lines.len() {
        let mut board = Board {
            cells: vec![],
            matched_cells_by_col: vec![0; 5],
            matched_cells_by_row: vec![0; 5],
            won: false,
        };
        for j in 0..5 {
            let row = lines[i + j]
                .split_whitespace()
                .map(|n| Cell {
                    num: n.parse::<u32>().expect(
                        format!("could not parse string to num on line {}", i + j).as_str(),
                    ),
                    marked: false,
                })
                .collect();
            board.cells.push(row);
        }
        boards.push(board);
        // skip empty line
        i += 6;
    }

    (draws, boards)
}

pub fn solve_1() -> u32 {
    let file = load_input_file("src/input/day4.txt");
    let (draws, mut boards) = parse_input(file);

    for draw in draws {
        for n in 0..boards.len() {
            let board = &mut boards[n];
            for (i, row) in board.cells.iter_mut().enumerate() {
                for (j, cell) in row.iter_mut().enumerate() {
                    if cell.num == draw {
                        cell.marked = true;
                        board.matched_cells_by_row[i] += 1;
                        board.matched_cells_by_col[j] += 1;

                        // win condition
                        if board.matched_cells_by_row[i] == 5 || board.matched_cells_by_col[j] == 5
                        {
                            return calc_winning_score(board, draw);
                        }
                    }
                }
            }
        }
    }

    0
}

pub fn solve_2() -> u32 {
    let file = load_input_file("src/input/day4.txt");
    let (draws, mut boards) = parse_input(file);
    let mut remaining = boards.len();

    for draw in draws {
        for n in 0..boards.len() {
            let board = &mut boards[n];
            if board.won {
                continue;
            }
            for (i, row) in board.cells.iter_mut().enumerate() {
                for (j, cell) in row.iter_mut().enumerate() {
                    if cell.num == draw {
                        cell.marked = true;
                        board.matched_cells_by_row[i] += 1;
                        board.matched_cells_by_col[j] += 1;

                        // win condition
                        if board.matched_cells_by_row[i] == 5 || board.matched_cells_by_col[j] == 5
                        {
                            board.won = true;
                            remaining -= 1;
                            if remaining == 0 {
                                return calc_winning_score(board, draw);
                            }
                        }
                    }
                }
            }
        }
    }

    0
}
