use adventofcode2021::get_stdin;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    println!("{}", solve(&input));

    Ok(())
}

fn solve(input: &str) -> i64 {
    let input = parse(input);
    let mut players = input.into_iter().map(Player::new).collect_vec();
    let mut roll_count = 0;
    let mut dice = Dice(1);
    'game: loop {
        for player in &mut players {
            player.play(&mut dice);
            roll_count += 3;
            if player.score >= 1000 {
                break 'game;
            }
        }
    }
    let loser_score = players.iter().map(|p| p.score).min().unwrap();
    loser_score * roll_count
}

struct Player {
    position: i64,
    score: i64,
}

impl Player {
    fn new(position: i64) -> Self {
        Self { position, score: 0 }
    }
    fn advance(&mut self, step: i64) {
        self.position = (self.position + step + 9) % 10 + 1;
        self.score += self.position;
    }
    fn play(&mut self, dice: &mut Dice) {
        let step = dice.roll();
        self.advance(step);
    }
}

struct Dice(i64);

impl Iterator for Dice {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.0;
        self.0 = ret % 100 + 1;
        Some(ret)
    }
}

impl Dice {
    fn roll(&mut self) -> i64 {
        (0..3).map(|_| self.next().unwrap()).sum()
    }
}

fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.rsplit_once(' ').unwrap().1.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll() {
        let mut dice = Dice(1);
        assert_eq!(dice.roll(), 6);
        assert_eq!(dice.roll(), 15);
        assert_eq!(dice.roll(), 24);
    }

    #[test]
    fn test_roll_100() {
        let mut dice = Dice(99);
        assert_eq!(dice.roll(), 99 + 100 + 1);
    }
}
