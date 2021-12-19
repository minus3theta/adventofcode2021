use std::{ops::AddAssign, str::FromStr};

use adventofcode2021::get_stdin;
use anyhow::bail;

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    println!("{}", solve(&input)?);

    Ok(())
}

fn solve(input: &str) -> anyhow::Result<i64> {
    let nums = input
        .lines()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(sum(&mut nums.into_iter()).magnitude())
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Token {
    Open,
    Close,
    Comma,
    Int(i64),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Number(Vec<Token>);

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.0 {
            match token {
                Token::Open => write!(f, "[")?,
                Token::Close => write!(f, "]")?,
                Token::Comma => write!(f, ",")?,
                Token::Int(x) => write!(f, "{}", x)?,
            }
        }
        Ok(())
    }
}

impl FromStr for Number {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Token::*;
        let mut v = Vec::new();
        let mut iter = s.chars().peekable();
        while let Some(c) = iter.next() {
            match c {
                '[' => v.push(Open),
                ']' => v.push(Close),
                ',' => v.push(Comma),
                '0'..='9' => {
                    let mut i = String::new();
                    i.push(c);
                    while let Some('0'..='9') = iter.peek() {
                        i.push(iter.next().unwrap());
                    }
                    v.push(Int(i.parse().unwrap()));
                }
                _ => bail!("unknown character: {}", c),
            }
        }
        Ok(Number(v))
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, mut rhs: Self) {
        use Token::*;
        self.0.insert(0, Open);
        self.0.push(Comma);
        self.0.append(&mut rhs.0);
        self.0.push(Close);
    }
}

fn sum(num: &mut impl Iterator<Item = Number>) -> Number {
    let mut sum = num.next().unwrap();
    for rhs in num {
        sum += rhs;
        sum.reduce();
    }
    sum
}

impl Number {
    fn explode(&mut self) -> bool {
        let mut depth = 0;
        for index in 0..self.0.len() {
            match self.0[index] {
                Token::Open => depth += 1,
                Token::Close => depth -= 1,
                Token::Comma => (),
                Token::Int(l) => {
                    if depth > 4 {
                        if let Token::Int(r) = self.0[index + 2] {
                            for target in (0..index).rev() {
                                if let Token::Int(ref mut x) = self.0[target] {
                                    *x += l;
                                    break;
                                }
                            }
                            for target in index + 3..self.0.len() {
                                if let Token::Int(ref mut x) = self.0[target] {
                                    *x += r;
                                    break;
                                }
                            }
                            self.0.splice(index - 1..index + 4, [Token::Int(0)]);
                            return true;
                        } else {
                            unreachable!()
                        }
                    }
                }
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        use Token::*;
        for index in 0..self.0.len() {
            match self.0[index] {
                Int(x) if x > 9 => {
                    let l = x / 2;
                    let r = x - l;
                    self.0
                        .splice(index..=index, [Open, Int(l), Comma, Int(r), Close]);
                    return true;
                }
                _ => (),
            }
        }
        false
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn magnitude(&self) -> i64 {
        fn go<'a>(iter: &mut impl Iterator<Item = &'a Token>) -> i64 {
            use Token::*;
            match iter.next() {
                Some(&Open) => {
                    let l = go(iter);
                    assert_eq!(iter.next(), Some(&Comma));
                    let r = go(iter);
                    assert_eq!(iter.next(), Some(&Close));
                    l * 3 + r * 2
                }
                Some(&Int(x)) => x,
                _ => unreachable!(),
            }
        }

        go(&mut self.0.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        use Token::*;
        assert_eq!(
            "[1,2]".parse::<Number>().unwrap(),
            Number([Open, Int(1), Comma, Int(2), Close].into_iter().collect())
        );
    }

    #[test]
    fn test_explode1() {
        let mut input: Number = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".parse().unwrap();
        input.explode();
        assert_eq!(input, "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]".parse().unwrap());
        input.explode();
        assert_eq!(input, "[[[[0,7],4],[15,[0,13]]],[1,1]]".parse().unwrap());
        assert!(!input.explode());
    }

    #[test]
    fn test_split() {
        let mut input: Number = "[[[[0,7],4],[15,[0,13]]],[1,1]]".parse().unwrap();
        assert!(input.split());
        assert_eq!(input, "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".parse().unwrap());
    }

    #[test]
    fn test_magnitude() {
        let input: Number = "[[1,2],[[3,4],5]]".parse().unwrap();
        assert_eq!(input.magnitude(), 143);
    }

    #[test]
    fn test_solve() {
        let input = r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(solve(input).unwrap(), 4140);
    }
}
