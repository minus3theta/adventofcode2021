use adventofcode2021::get_stdin;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Position {
    Open,
    Close,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Bracket {
    Paren,
    Square,
    Wave,
    Angle,
}

impl Bracket {
    pub fn score(&self) -> i64 {
        match self {
            Paren => 1,
            Square => 2,
            Wave => 3,
            Angle => 4,
        }
    }
}

use itertools::Itertools;
use Bracket::*;
use Position::*;

fn detect(c: char) -> (Position, Bracket) {
    match c {
        '(' => (Open, Paren),
        ')' => (Close, Paren),
        '[' => (Open, Square),
        ']' => (Close, Square),
        '{' => (Open, Wave),
        '}' => (Close, Wave),
        '<' => (Open, Angle),
        '>' => (Close, Angle),
        _ => unreachable!(),
    }
}

fn complete(line: &str) -> Option<Vec<Bracket>> {
    let mut stack = vec![];
    for c in line.chars() {
        match detect(c) {
            (Open, bracket) => {
                stack.push(bracket);
            }
            (Close, bracket) => match stack.pop() {
                Some(open) if open == bracket => (),
                _ => return None,
            },
        }
    }
    Some(stack)
}

fn score_to_complete(seq: &[Bracket]) -> i64 {
    let mut ret = 0;
    for b in seq.iter().rev() {
        ret *= 5;
        ret += b.score();
    }
    ret
}

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let mut incomplete = input
        .lines()
        .filter_map(|l| complete(l).map(|seq| score_to_complete(&seq)))
        .collect_vec();
    incomplete.sort_unstable();
    println!("{}", incomplete[incomplete.len() / 2]);

    Ok(())
}
