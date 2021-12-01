use std::io::{self, Read};

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    lock.read_to_string(&mut input)?;

    let input = parse(&input);
    let ans = solve(&input);
    println!("{}", ans);

    Ok(())
}

fn parse(input: &str) -> Vec<i32> {
    input.split('\n').map(|s| s.parse().unwrap()).collect()
}

fn solve(input: &[i32]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(&x, &y, &z)| x + y + z)
        .tuple_windows()
        .filter(|(x, y)| x < y)
        .count()
}
