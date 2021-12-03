use adventofcode2021::get_stdin;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let input = input.lines().map(|s| s.chars().collect_vec()).collect_vec();

    let len = input[0].len();
    let mut ones = vec![0; len];
    let mut zeros = vec![0; len];
    for v in &input {
        for (j, &c) in v.iter().enumerate() {
            if c == '1' {
                ones[j] += 1;
            } else {
                zeros[j] += 1;
            }
        }
    }

    let mut gamma = 0;
    let mut eps = 0;
    for (&o, &z) in ones.iter().zip(&zeros) {
        gamma <<= 1;
        eps <<= 1;
        if o > z {
            gamma += 1;
        } else {
            eps += 1;
        }
    }
    println!("{}", gamma * eps);

    Ok(())
}
