use adventofcode2021::get_stdin;
use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

static INPUT_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)$").unwrap());

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let target = parse(&input)?;
    println!("{}", solve(target));

    Ok(())
}

type Area = ((i64, i64), (i64, i64));

fn parse(input: &str) -> anyhow::Result<Area> {
    let cap = INPUT_PATTERN
        .captures_iter(input)
        .next()
        .context("no match")?;
    Ok((
        (cap[1].parse()?, cap[2].parse()?),
        (cap[3].parse()?, cap[4].parse()?),
    ))
}

fn solve(target: Area) -> usize {
    let ((_, x1), (y0, _)) = target;
    (0..=x1)
        .flat_map(|vx| (-y0.abs()..=y0.abs()).map(move |vy| top(target, vx, vy)))
        .filter(|&x| x)
        .count()
}

fn top(target: Area, mut vx: i64, mut vy: i64) -> bool {
    let mut x = 0;
    let mut y = 0;

    loop {
        if is_inside(x, y, target) {
            return true;
        }
        if is_below(y, target) {
            return false;
        }
        x += vx;
        y += vy;
        use std::cmp::Ordering;
        match vx.cmp(&0) {
            Ordering::Greater => {
                vx -= 1;
            }
            Ordering::Less => {
                vx += 1;
            }
            _ => (),
        }
        vy -= 1;
    }
}

fn is_inside(x: i64, y: i64, target: Area) -> bool {
    let ((x0, x1), (y0, y1)) = target;
    x0 <= x && x <= x1 && y0 <= y && y <= y1
}

fn is_below(y: i64, target: Area) -> bool {
    let (_, (y0, _)) = target;
    y < y0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(parse(input).unwrap(), ((20, 30), (-10, -5)));
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(((20, 30), (-10, -5))), 112);
    }
}
