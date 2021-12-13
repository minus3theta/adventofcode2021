use std::collections::BTreeSet;

use adventofcode2021::get_stdin;
use anyhow::Context;
use itertools::Itertools;

type Field = BTreeSet<(i32, i32)>;
type Operation<'a> = (&'a str, i32);

fn parse(input: &str) -> anyhow::Result<(Field, Vec<Operation>)> {
    let mut dots = BTreeSet::new();
    let mut ops = vec![];
    for l in input.lines() {
        if l.is_empty() {
            continue;
        }
        if l.contains(',') {
            let xy = l
                .split(',')
                .map(|s| s.parse())
                .collect::<Result<Vec<i32>, _>>()?;
            dots.insert((xy[0], xy[1]));
        } else if l.starts_with("fold") {
            let s = l
                .split_ascii_whitespace()
                .nth(2)
                .context("fold syntax error")?;
            let (dir, pos) = s.split('=').collect_tuple().context("fold syntax error")?;
            let pos = pos.parse()?;
            ops.push((dir, pos));
        }
    }

    Ok((dots, ops))
}

fn fold_x(field: &Field, pos: i32) -> Field {
    field
        .iter()
        .map(|&(x, y)| {
            if x > pos {
                (pos - (x - pos), y)
            } else {
                (x, y)
            }
        })
        .collect()
}

fn fold_y(field: &Field, pos: i32) -> Field {
    field
        .iter()
        .map(|&(x, y)| {
            if y > pos {
                (x, pos - (y - pos))
            } else {
                (x, y)
            }
        })
        .collect()
}

fn solve(mut field: Field, ops: &[Operation]) -> Field {
    for &(dir, pos) in ops {
        field = match dir {
            "x" => fold_x(&field, pos),
            "y" => fold_y(&field, pos),
            _ => unreachable!(),
        };
    }
    field
}

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let (field, ops) = parse(&input)?;

    let field = solve(field, &ops);
    let width = field
        .iter()
        .map(|&(x, _)| x)
        .max()
        .context("no dot found")?;
    let height = field
        .iter()
        .map(|&(_, y)| y)
        .max()
        .context("no dot found")?;
    for y in 0..=height {
        for x in 0..=width {
            print!("{}", if field.contains(&(x, y)) { "#" } else { "." });
        }
        println!();
    }

    Ok(())
}
