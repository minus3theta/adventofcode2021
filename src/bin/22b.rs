use std::collections::HashMap;

use adventofcode2021::get_stdin;
use indicatif::ProgressIterator;
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

fn solve(input: &str) -> i64 {
    let input = parse(input);
    let width_range = (0..3)
        .map(|i| compress(input.iter().map(|(_, ranges)| &ranges[i])))
        .collect_vec();
    let mut lights = vec![
        vec![vec![false; width_range[2].0.len()]; width_range[1].0.len()];
        width_range[0].0.len()
    ];
    for (op, ranges) in input.into_iter().progress() {
        for x in clip(ranges[0], &width_range[0].1) {
            for y in clip(ranges[1], &width_range[1].1) {
                for z in clip(ranges[2], &width_range[2].1) {
                    lights[x][y][z] = op;
                }
            }
        }
    }
    let mut ans = 0;
    for (x, plane) in lights.iter().enumerate() {
        for (y, line) in plane.iter().enumerate() {
            for (z, &f) in line.iter().enumerate() {
                if f {
                    ans += width_range[0].0[x] * width_range[1].0[y] * width_range[2].0[z];
                }
            }
        }
    }
    ans
}

fn compress<'a>(ranges: impl Iterator<Item = &'a (i64, i64)>) -> (Vec<i64>, HashMap<i64, usize>) {
    let coords = ranges
        .flat_map(|&(l, r)| [l, r].into_iter())
        .sorted_unstable()
        .dedup()
        .collect_vec();
    let width = coords
        .iter()
        .map(|_| 1)
        .interleave(coords.iter().tuple_windows().map(|(&l, &r)| r - l - 1))
        .collect();
    let lookup = coords
        .iter()
        .enumerate()
        .map(|(i, &c)| (c, 2 * i))
        .collect();
    (width, lookup)
}

fn clip(range: (i64, i64), lookup: &HashMap<i64, usize>) -> impl Iterator<Item = usize> {
    lookup[&range.0]..=lookup[&range.1]
}
