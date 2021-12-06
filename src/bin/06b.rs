use adventofcode2021::get_stdin;

fn step(popl: &mut [i64]) {
    let spawn = popl[0];
    for i in 0..8 {
        popl[i] = popl[i + 1];
    }
    popl[8] = spawn;
    popl[6] += spawn;
}

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let mut popl = vec![0; 9];
    for fish in input.split(',').map(|s| s.parse::<usize>()).flatten() {
        popl[fish] += 1;
    }

    for _ in 0..256 {
        step(&mut popl);
    }
    println!("{}", popl.iter().sum::<i64>());

    Ok(())
}
