use adventofcode2021::get_stdin;
use itertools::Itertools;

const DIRS: [(usize, usize); 4] = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let map = input
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect_vec())
        .collect_vec();
    let h = map.len();
    let w = map[0].len();

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if DIRS
                .iter()
                .filter_map(|&(di, dj)| {
                    let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
                    if ni < h && nj < w {
                        Some(map[ni][nj])
                    } else {
                        None
                    }
                })
                .all(|v| v > map[i][j])
            {
                ans += map[i][j] as i64 + 1;
            }
        }
    }
    println!("{}", ans);

    Ok(())
}
