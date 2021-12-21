use adventofcode2021::get_stdin;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    println!("{}", solve(&input));

    Ok(())
}

const MAX_SCORE: usize = 21;

fn solve(input: &str) -> i64 {
    let input = parse(input);
    // universe[score_0][score_1][position_0][position_1][turn]
    let mut universe = vec![vec![vec![vec![vec![0; 2]; 11]; 11]; MAX_SCORE + 1]; MAX_SCORE + 1];
    universe[0][0][input[0]][input[1]][0] = 1;
    for score_0 in 0..MAX_SCORE {
        for score_1 in 0..MAX_SCORE {
            for position_0 in 1..=10 {
                for position_1 in 1..=10 {
                    for turn in 0..2 {
                        let current = universe[score_0][score_1][position_0][position_1][turn];
                        for step in (0..3).map(|_| (1..=3)).multi_cartesian_product() {
                            let step = step.into_iter().sum();
                            let (n_position_0, n_position_1) = if turn == 0 {
                                (advance(position_0, step), position_1)
                            } else {
                                (position_0, advance(position_1, step))
                            };
                            let (n_score_0, n_score_1) = if turn == 0 {
                                (add_score(score_0, n_position_0), score_1)
                            } else {
                                (score_0, add_score(score_1, n_position_1))
                            };
                            universe[n_score_0][n_score_1][n_position_0][n_position_1][1 - turn] +=
                                current;
                        }
                    }
                }
            }
        }
    }
    let win_0 = universe[MAX_SCORE]
        .iter()
        .flat_map(|v| v.iter().flat_map(|v| v.iter().flat_map(|v| v.iter())))
        .sum::<i64>();
    let win_1 = universe
        .iter()
        .flat_map(|v| {
            v[MAX_SCORE]
                .iter()
                .flat_map(|v| v.iter().flat_map(|v| v.iter()))
        })
        .sum::<i64>();
    win_0.max(win_1)
}

fn advance(position: usize, step: usize) -> usize {
    (position + step + 9) % 10 + 1
}

fn add_score(score: usize, gain: usize) -> usize {
    (score + gain).min(MAX_SCORE)
}

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| l.rsplit_once(' ').unwrap().1.parse().unwrap())
        .collect()
}
