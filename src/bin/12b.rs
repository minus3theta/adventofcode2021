use std::collections::{BTreeMap, BTreeSet};

use adventofcode2021::get_stdin;
use itertools::Itertools;

fn dfs(
    adj: &BTreeMap<&str, BTreeSet<&str>>,
    v: &str,
    visited: BTreeSet<&str>,
    twice: bool,
) -> usize {
    let mut ret = 0;
    for &u in &adj[v] {
        if u == "start" {
            continue;
        }
        if u == "end" {
            ret += 1;
        } else {
            let u_small = u.chars().all(|c| c.is_ascii_lowercase());
            let is_second = u_small && visited.contains(u);
            if is_second && twice {
                continue;
            }
            let mut new_visited = visited.clone();
            new_visited.insert(u);
            ret += dfs(adj, u, new_visited, twice || is_second);
        }
    }
    ret
}

fn solve(input: &str) -> usize {
    let mut adj = BTreeMap::new();
    for line in input.lines() {
        let (a, b) = line.split('-').collect_tuple().unwrap();
        adj.entry(a).or_insert_with(BTreeSet::new).insert(b);
        adj.entry(b).or_insert_with(BTreeSet::new).insert(a);
    }
    dfs(&adj, "start", BTreeSet::new(), false)
}

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;

    println!("{}", solve(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(solve(input), 36);
    }

    #[test]
    fn sample_2() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        assert_eq!(solve(input), 103);
    }

    #[test]
    fn sample_3() {
        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        assert_eq!(solve(input), 3509);
    }
}
