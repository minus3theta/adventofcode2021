use std::str::FromStr;

use adventofcode2021::get_stdin;
use anyhow::bail;

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let mut field: Field = input.parse()?;
    println!("{}", field.solve());

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cell(Option<SC>);

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use SC::*;
        Ok(Cell(match value {
            '.' => None,
            '>' => Some(East),
            'v' => Some(South),
            _ => bail!("{}", value),
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SC {
    East,
    South,
}

impl SC {
    fn forward(&self, r: usize, c: usize, h: usize, w: usize) -> (usize, usize) {
        match self {
            SC::East => (r, (c + 1) % w),
            SC::South => ((r + 1) % h, c),
        }
    }
}

struct Field(Vec<Vec<Cell>>);

impl FromStr for Field {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Field(
            s.lines()
                .map(|l| l.chars().map(Cell::try_from).collect())
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl Field {
    fn solve(&mut self) -> i32 {
        for steps in 1.. {
            if !self.step() {
                return steps;
            }
        }
        unreachable!()
    }

    fn step(&mut self) -> bool {
        let e = self.step_at(SC::East);
        let s = self.step_at(SC::South);
        e || s
    }

    fn step_at(&mut self, s: SC) -> bool {
        let h = self.0.len();
        let w = self.0[0].len();
        let mut updates = vec![];
        for (r, row) in self.0.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if let Some(sc) = cell.0 {
                    if sc == s {
                        let (fr, fc) = sc.forward(r, c, h, w);
                        if self.0[fr][fc].0.is_none() {
                            updates.push((r, c, fr, fc));
                        }
                    }
                }
            }
        }
        if updates.is_empty() {
            return false;
        }
        for (r, c, fr, fc) in updates {
            self.0[r][c] = Cell(None);
            self.0[fr][fc] = Cell(Some(s));
        }
        true
    }
}
