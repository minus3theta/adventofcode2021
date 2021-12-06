use adventofcode2021::get_stdin;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let input = input
        .lines()
        .filter_map(|l| {
            l.split(" -> ")
                .filter_map(|s| {
                    s.split(',')
                        .filter_map(|s| s.parse().ok())
                        .collect_tuple::<(usize, usize)>()
                })
                .collect_tuple::<(_, _)>()
        })
        .collect_vec();
    println!("{}", solve(&input));

    Ok(())
}

fn solve(input: &[((usize, usize), (usize, usize))]) -> usize {
    let mut field = vec![vec![0; 1000]; 1000];
    for &((x, y), (z, w)) in input {
        let v = (z as isize - x as isize, w as isize - y as isize);
        let len = v.0.abs().max(v.1.abs());
        let v = (v.0 / len, v.1 / len);
        let mut p = (x, y);
        while p != (z, w) {
            field[p.0][p.1] += 1;
            p.0 = (p.0 as isize + v.0) as usize;
            p.1 = (p.1 as isize + v.1) as usize;
        }
        field[p.0][p.1] += 1;
    }
    field
        .iter()
        .flat_map(|r| r.iter())
        .filter(|&&c| c > 1)
        .count()
}
