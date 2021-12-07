use adventofcode2021::get_stdin;

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let mut position = input
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<i32>, _>>()?;
    position.sort_unstable();
    let index = position.len() / 2;
    let median = position[index];
    let total_fuel = position.iter().map(|&p| (p - median).abs()).sum::<i32>();

    println!("{}", total_fuel);

    Ok(())
}
