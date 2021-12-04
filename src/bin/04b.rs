use adventofcode2021::get_stdin;
use anyhow::Context;
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Cell {
    num: i32,
    marked: bool,
}

impl Cell {
    fn new(num: i32) -> Self {
        Self { num, marked: false }
    }
}

#[derive(Clone, Debug)]
struct Board {
    board: Vec<Vec<Cell>>,
    bingo: bool,
}

impl Board {
    fn new(board: Vec<Vec<Cell>>) -> Self {
        Self {
            board,
            bingo: false,
        }
    }

    fn mark(&mut self, num: i32) {
        for row in &mut self.board {
            for cell in row {
                if num == cell.num {
                    cell.marked = true;
                }
            }
        }
    }

    fn is_bingo(&self) -> bool {
        for row in &self.board {
            if row.iter().all(|c| c.marked) {
                return true;
            }
        }
        for i in 0..5 {
            if self.board.iter().all(|row| row[i].marked) {
                return true;
            }
        }
        false
    }

    fn sum_unmarked(&self) -> i32 {
        self.board
            .iter()
            .flat_map(|row| row.iter())
            .filter_map(|c| if c.marked { None } else { Some(c.num) })
            .sum()
    }
}

fn losers_score(boards: &mut [Board], called: &[i32]) -> i32 {
    let count_board = boards.len();
    let mut filled = 0;
    for &num in called {
        for board in boards.iter_mut() {
            if board.bingo {
                continue;
            }
            board.mark(num);
            if board.is_bingo() {
                board.bingo = true;
                filled += 1;
                if filled == count_board {
                    return board.sum_unmarked() * num;
                }
            }
        }
    }
    unreachable!()
}

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let mut lines = input.lines();
    let called = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<i32>, _>>()?;
    let mut boards = vec![];
    for board in &lines.into_iter().chunks(6) {
        let b = Board::new(
            board
                .into_iter()
                .skip(1)
                .map(|row| {
                    row.split_ascii_whitespace()
                        .map(|s| Some(Cell::new(s.parse().ok()?)))
                        .collect::<Option<Vec<_>>>()
                })
                .collect::<Option<Vec<_>>>()
                .context("parse error")?,
        );
        boards.push(b);
    }
    println!("{}", losers_score(&mut boards, &called));

    Ok(())
}
