use adventofcode2021::get_stdin;

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let mut horizontal = 0;
    let mut depth = 0;
    for line in input.lines() {
        let line = line.split_ascii_whitespace().collect_vec();
        let size: i32 = line[1].parse()?;
        match line[0] {
            "forward" => horizontal += size,
            "down" => depth += size,
            "up" => depth -= size,
            _ => unreachable!(),
        }
    }
    println!("{}", horizontal * depth);

    Ok(())
}
