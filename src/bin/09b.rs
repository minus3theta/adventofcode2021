use adventofcode2021::get_stdin;
use itertools::Itertools;

use union_find_with_size::*;
pub mod union_find_with_size {
    #[derive(Clone, Debug)]
    pub struct UnionFindWithSize {
        par: Vec<usize>,
        size: Vec<usize>,
    }
    impl UnionFindWithSize {
        pub fn new(n: usize) -> Self {
            let mut uf = UnionFindWithSize {
                par: vec![0; n],
                size: vec![1; n],
            };
            for (i, item) in uf.par.iter_mut().enumerate() {
                *item = i;
            }
            uf
        }
        pub fn find(&mut self, x: usize) -> usize {
            if self.par[x] == x {
                x
            } else {
                let px = self.par[x];
                self.par[x] = self.find(px);
                self.par[x]
            }
        }
        pub fn unite(&mut self, x: usize, y: usize) {
            let x = self.find(x);
            let y = self.find(y);
            if x == y {
                return;
            }
            if self.size[x] < self.size[y] {
                self.par[x] = y;
                self.size[y] += self.size[x];
            } else {
                self.par[y] = x;
                self.size[x] += self.size[y];
            }
        }
        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }
        pub fn size(&mut self, x: usize) -> usize {
            let root = self.find(x);
            self.size[root]
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let map = input
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect_vec())
        .collect_vec();
    let h = map.len();
    let w = map[0].len();

    let mut uf = UnionFindWithSize::new(h * w);
    for i in 0..h {
        for j in 0..w - 1 {
            if map[i][j] != 9 && map[i][j + 1] != 9 {
                uf.unite(i * w + j, i * w + j + 1);
            }
        }
    }
    for i in 0..h - 1 {
        for j in 0..w {
            if map[i][j] != 9 && map[i + 1][j] != 9 {
                uf.unite(i * w + j, (i + 1) * w + j);
            }
        }
    }

    let mut size = vec![];
    for x in 0..h * w {
        if uf.find(x) == x {
            size.push(uf.size(x));
        }
    }
    size.sort_unstable();

    println!("{}", size.iter().rev().take(3).product::<usize>());

    Ok(())
}
