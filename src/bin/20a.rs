use std::collections::BTreeSet;

use adventofcode2021::get_stdin;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let (algo, picture) = parse(&input);
    let picture = apply(&algo, &picture, false);
    let picture = apply(&algo, &picture, true);
    dbg!(picture.len());

    Ok(())
}

type Picture = BTreeSet<(i64, i64)>;

fn parse(input: &str) -> (Vec<bool>, Picture) {
    let mut iter = input.lines();

    let algo = iter.next().unwrap();
    let algo = algo.chars().map(|c| c == '#').collect();
    let _ = iter.next();

    let picture = iter
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| (c == '#').then(|| (row as i64, col as i64)))
        })
        .collect();

    (algo, picture)
}

fn index(picture: &Picture, row: i64, col: i64, flip: bool) -> usize {
    let mut ret = 0;
    for r in row - 1..=row + 1 {
        for c in col - 1..=col + 1 {
            ret <<= 1;
            if picture.contains(&(r, c)) ^ flip {
                ret += 1;
            }
        }
    }
    ret
}

fn adjacent(picture: &Picture) -> Picture {
    picture
        .iter()
        .flat_map(|&(r, c)| (r - 1..=r + 1).cartesian_product(c - 1..=c + 1))
        .collect()
}

fn apply(algo: &[bool], picture: &Picture, flip: bool) -> Picture {
    let mut new_picture = Picture::new();
    for &(r, c) in &adjacent(picture) {
        let i = index(picture, r, c, flip);
        let color = algo[i];
        let next_flip = !flip;
        // let next_flip = flip;
        if color ^ next_flip {
            new_picture.insert((r, c));
        }
    }

    new_picture
}

fn _show(picture: &Picture) {
    for r in -5..10 {
        for c in -5..10 {
            eprint!("{}", if picture.contains(&(r, c)) { '#' } else { '.' });
        }
        eprintln!();
    }
}
