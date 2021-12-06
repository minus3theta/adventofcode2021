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
        if x == z {
            for i in y.min(w)..=y.max(w) {
                field[x][i] += 1;
            }
        } else if y == w {
            for i in x.min(z)..=x.max(z) {
                field[i][y] += 1;
            }
        }
    }
    field
        .iter()
        .flat_map(|r| r.iter())
        .filter(|&&c| c > 1)
        .count()
}
