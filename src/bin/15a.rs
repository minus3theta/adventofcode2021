use std::{cmp::Reverse, collections::BinaryHeap};

use adventofcode2021::get_stdin;

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c as i32 - '0' as i32).collect())
        .collect()
}

const DIRS: [(usize, usize); 4] = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];

fn solve(map: &[Vec<i32>]) -> i32 {
    let h = map.len();
    let w = map[0].len();
    let mut dist = vec![vec![1 << 30; w]; h];
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), (0usize, 0usize)));

    while let Some((Reverse(d), (i, j))) = que.pop() {
        if dist[i][j] < d {
            continue;
        }
        for &(di, dj) in &DIRS {
            let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
            if ni < h && nj < w {
                let c = map[ni][nj];
                if d + c < dist[ni][nj] {
                    dist[ni][nj] = d + c;
                    que.push((Reverse(d + c), (ni, nj)));
                }
            }
        }
    }

    dist[h - 1][w - 1]
}

fn main() -> anyhow::Result<()> {
    let map = parse(&get_stdin()?);
    let ans = solve(&map);
    println!("{}", ans);

    Ok(())
}
