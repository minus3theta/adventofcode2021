use adventofcode2021::get_stdin;
use anyhow::Context;
use itertools::Itertools;

fn fuel(position: i32, target: i32) -> i32 {
    let dist = (position - target).abs();
    dist * (dist + 1) / 2
}

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let position = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<i32>, _>>()?;
    let (&min, &max) = position
        .iter()
        .minmax()
        .into_option()
        .context("position is empty")?;
    let min_fuel = (min..=max)
        .map(|target| position.iter().map(|&p| fuel(p, target)).sum::<i32>())
        .min()
        .unwrap();

    println!("{}", min_fuel);

    Ok(())
}
