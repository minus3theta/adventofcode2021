use std::collections::BTreeSet;

use adventofcode2021::get_stdin;
use itertools::Itertools;

fn step(octo: &mut [Vec<i32>]) -> usize {
    let h = octo.len();
    let w = octo[0].len();

    let mut flashing = vec![];
    let mut flashed = BTreeSet::new();
    for (i, row) in octo.iter_mut().enumerate() {
        for (j, c) in row.iter_mut().enumerate() {
            *c += 1;
            if *c > 9 {
                flashing.push((i, j));
                flashed.insert((i, j));
            }
        }
    }
    while !flashing.is_empty() {
        let mut to_flash = vec![];
        for (i, j) in flashing {
            for ni in i.saturating_sub(1)..(i + 2).min(h) {
                for nj in j.saturating_sub(1)..(j + 2).min(w) {
                    if (ni, nj) == (i, j) {
                        continue;
                    }
                    octo[ni][nj] += 1;
                    if octo[ni][nj] > 9 && !flashed.contains(&(ni, nj)) {
                        to_flash.push((ni, nj));
                        flashed.insert((ni, nj));
                    }
                }
            }
        }
        flashing = to_flash;
    }
    for &(i, j) in &flashed {
        octo[i][j] = 0;
    }
    flashed.len()
}

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let mut octo = input
        .lines()
        .map(|l| l.chars().map(|c| c as i32 - '0' as i32).collect_vec())
        .collect_vec();

    let mut sum = 0;
    for _ in 0..100 {
        sum += step(&mut octo);
    }
    println!("{}", sum);

    Ok(())
}
