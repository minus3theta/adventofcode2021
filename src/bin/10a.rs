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
            Paren => 3,
            Square => 57,
            Wave => 1197,
            Angle => 25137,
        }
    }
}

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

fn is_corrupted(line: &str) -> Option<Bracket> {
    let mut stack = vec![];
    for c in line.chars() {
        match detect(c) {
            (Open, bracket) => {
                stack.push(bracket);
            }
            (Close, bracket) => match stack.pop() {
                Some(open) if open == bracket => (),
                _ => return Some(bracket),
            },
        }
    }
    None
}

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let ans: i64 = input
        .lines()
        .map(|l| is_corrupted(l).map_or(0, |b| b.score()))
        .sum();
    println!("{}", ans);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corrupted() {
        assert_eq!(is_corrupted("[<>({}){}[([])<>]]"), None);
        assert_eq!(is_corrupted("{([(<{}[<>[]}>{[]{[(<()>"), Some(Wave));
    }
}
