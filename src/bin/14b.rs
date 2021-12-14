use std::collections::BTreeMap;

use adventofcode2021::get_stdin;
use anyhow::Context;
use itertools::Itertools;

type Count = BTreeMap<char, usize>;
type PairCount = BTreeMap<(char, char), usize>;
type Formula = BTreeMap<(char, char), char>;

fn parse(input: &str) -> Option<(Count, PairCount, Formula)> {
    let mut lines = input.lines();

    let template = lines.next()?;
    let mut count = Count::new();
    for x in template.chars() {
        *count.entry(x).or_insert(0) += 1;
    }
    let mut pair_count = PairCount::new();
    for (a, b) in template.chars().tuple_windows() {
        *pair_count.entry((a, b)).or_insert(0) += 1;
    }

    lines.next();

    let mut formulae = Formula::new();
    for line in lines {
        let (ab, c) = line.split(" -> ").collect_tuple()?;
        let (a, b) = ab.chars().collect_tuple()?;
        let (c,) = c.chars().collect_tuple()?;
        formulae.insert((a, b), c);
    }

    Some((count, pair_count, formulae))
}

fn polymerize(
    count: &mut Count,
    pair_count: &PairCount,
    formulae: &Formula,
) -> anyhow::Result<PairCount> {
    let mut new_pair_count = PairCount::new();
    for (&(a, b), &n) in pair_count {
        let new = *formulae.get(&(a, b)).context("no such formula")?;
        *count.entry(new).or_insert(0) += n;
        *new_pair_count.entry((a, new)).or_insert(0) += n;
        *new_pair_count.entry((new, b)).or_insert(0) += n;
    }

    Ok(new_pair_count)
}

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let (mut count, mut pair_count, formulae) = parse(&input).context("parse error")?;
    for _ in 0..40 {
        pair_count = polymerize(&mut count, &pair_count, &formulae)?;
    }
    let (&min, &max) = count
        .values()
        .minmax()
        .into_option()
        .context("no elements")?;
    println!("{}", max - min);

    Ok(())
}
