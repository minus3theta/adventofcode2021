use std::{collections::HashMap, str::FromStr};

use adventofcode2021::get_stdin;
use anyhow::{bail, Context};
use either::Either;
use indicatif::ProgressIterator;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    let ans = solve(&input)?;
    for &d in &ans {
        print!("{}", d);
    }
    println!();

    Ok(())
}

fn solve(input: &str) -> anyhow::Result<Vec<i32>> {
    let instructions: Vec<_> = input
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<_, _>>()?;
    let mut state_to_input: HashMap<State, Vec<i32>> = [Default::default()].into_iter().collect();
    for ins in instructions.iter().progress() {
        let mut next: HashMap<State, Vec<i32>> = Default::default();
        for (state, input) in state_to_input {
            for (state, i) in state.apply(ins) {
                let mut input = input.clone();
                if let Some(i) = i {
                    input.push(i);
                }
                next.entry(state)
                    .and_modify(|i| *i = i.max(&mut input).to_vec())
                    .or_insert(input);
            }
        }
        state_to_input = next;
    }

    state_to_input
        .into_iter()
        .filter_map(|(state, input)| (state.value_var(&Variable(3)) == 0).then(|| input))
        .max()
        .context("no such state")
}

enum Instruction {
    Inp(Variable),
    Op(Operation, Variable, Operand),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;
        let mut iter = s.split_ascii_whitespace();
        Ok(match iter.next().context("no opcode")? {
            "inp" => {
                let (var,) = iter.collect_tuple().context("invalid operand")?;
                Inp(var.parse()?)
            }
            op => {
                let (a, b) = iter
                    .collect_tuple()
                    .context("invalid number of arguments")?;
                Op(op.parse()?, a.parse()?, b.parse()?)
            }
        })
    }
}

enum Operation {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Operation::*;
        Ok(match s {
            "add" => Add,
            "mul" => Mul,
            "div" => Div,
            "mod" => Mod,
            "eql" => Eql,
            _ => bail!("invalid operation: {}", s),
        })
    }
}

enum Operand {
    Var(Variable),
    Int(i32),
}

impl FromStr for Operand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "w" | "x" | "y" | "z" => Operand::Var(s.parse()?),
            _ => Operand::Int(s.parse()?),
        })
    }
}

struct Variable(usize);

impl FromStr for Variable {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Variable(match s {
            "w" => 0,
            "x" => 1,
            "y" => 2,
            "z" => 3,
            _ => bail!("invalid variable: {}", s),
        }))
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct State([i32; 4]);

impl State {
    fn value_var(&self, var: &Variable) -> i32 {
        self.0[var.0]
    }

    fn value(&self, o: &Operand) -> i32 {
        use Operand::*;
        match o {
            Var(v) => self.0[v.0],
            &Int(i) => i,
        }
    }

    fn update(&self, var: &Variable, value: i32) -> Self {
        let mut state = self.clone();
        state.0[var.0] = value;
        state
    }

    fn apply<'a>(&'a self, ins: &'a Instruction) -> impl Iterator<Item = (Self, Option<i32>)> + 'a {
        use Instruction::*;
        use Operation::*;
        match ins {
            Inp(v) => Either::Left((1..=9).map(|i| (self.update(v, i), Some(i)))),
            Op(op, o1, o2) => {
                let v1 = self.value_var(o1);
                let v2 = self.value(o2);
                let new_value = match op {
                    Add => Some(v1 + v2),
                    Mul => Some(v1 * v2),
                    Div => v1.checked_div(v2),
                    Mod => (!(v1 < 0 || v2 <= 0)).then(|| v1 % v2),
                    Eql => Some(if v1 == v2 { 1 } else { 0 }),
                };
                Either::Right(new_value.map(|v| (self.update(o1, v), None)).into_iter())
            }
        }
    }
}
