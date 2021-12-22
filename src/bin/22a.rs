use std::collections::HashSet;

use adventofcode2021::get_stdin;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    println!("{}", solve(&input));

    Ok(())
}

fn parse(input: &str) -> Vec<(bool, Vec<(i64, i64)>)> {
    input
        .lines()
        .map(|l| {
            let (on, ranges) = l.split_whitespace().collect_tuple().unwrap();
            let ranges = ranges
                .split(',')
                .map(|s| {
                    let (_, range) = s.rsplit_once("=").unwrap();
                    let (begin, end) = range.split_once("..").unwrap();
                    (begin.parse::<i64>().unwrap(), end.parse::<i64>().unwrap())
                })
                .collect();
            (on == "on", ranges)
        })
        .collect()
}

fn solve(input: &str) -> usize {
    let input = parse(input);
    let mut lights = HashSet::new();
    for (op, ranges) in input {
        for x in clip(ranges[0]) {
            for y in clip(ranges[1]) {
                for z in clip(ranges[2]) {
                    let pos = (x, y, z);
                    if op {
                        lights.insert(pos);
                    } else {
                        lights.remove(&pos);
                    }
                }
            }
        }
    }
    lights.len()
}

fn clip(range: (i64, i64)) -> impl Iterator<Item = i64> {
    range.0.max(-50)..=range.1.min(50)
}
