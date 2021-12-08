use std::collections::BTreeSet;

use adventofcode2021::get_stdin;
use itertools::Itertools;
use once_cell::sync::Lazy;

static DIGITS: Lazy<Vec<Digit>> = Lazy::new(|| {
    [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .into_iter()
    .map(str_to_index)
    .collect()
});

type Pat = Vec<usize>;
type PatRef<'a> = &'a [usize];
type Digit = BTreeSet<usize>;

fn translate(pat: PatRef, digit: &Digit) -> Option<usize> {
    let t: Digit = digit.iter().map(|&d| pat[d]).collect();
    DIGITS.iter().find_position(|&d| d == &t).map(|(i, _)| i)
}

fn find_pat(display: &[Digit]) -> Pat {
    for pat in (0..7).permutations(7) {
        if display.iter().all(|d| translate(&pat, d).is_some()) {
            return pat;
        }
    }
    unreachable!()
}

fn alpha_to_index(c: char) -> usize {
    (c as u32 - 'a' as u32) as usize
}

fn str_to_index(s: &str) -> BTreeSet<usize> {
    s.chars().map(alpha_to_index).collect()
}

fn parse_display(display: &str) -> Vec<Digit> {
    display.split_ascii_whitespace().map(str_to_index).collect()
}

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let mut sum = 0i64;
    for line in input.lines() {
        let line = line.split(" | ").collect_vec();
        let display = parse_display(line[0]);
        let pat = find_pat(&display);
        let ret = parse_display(line[1]);
        let mut num = 0;
        for r in &ret {
            num *= 10;
            num += translate(&pat, r).unwrap() as i64;
        }
        sum += num;
    }
    dbg!(sum);

    Ok(())
}
