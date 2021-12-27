use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use adventofcode2021::get_stdin;
use anyhow::bail;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = get_stdin()?;
    println!("{}", solve(&input));

    Ok(())
}

fn parse(input: &str) -> State {
    let (top, bottom) = input
        .lines()
        .skip(2)
        .take(2)
        .map(|line| line.chars().filter_map(|c| Amphi::try_from(c).ok()))
        .collect_tuple()
        .unwrap();
    let rooms = top
        .zip(bottom)
        .map(|(t, b)| Room::Two(t, b))
        .collect_vec()
        .try_into()
        .unwrap();
    State::new(rooms)
}

fn solve(input: &str) -> i64 {
    let init: State = parse(input);

    let mut dist = HashMap::<State, i64>::new();
    dist.insert(init.clone(), 0);

    let mut que = BinaryHeap::new();
    que.push((Reverse(0), init));

    while let Some((Reverse(d), state)) = que.pop() {
        if state.is_goal() {
            return d;
        }
        if dist[&state] < d {
            continue;
        }
        for (cost, next_state) in state.next() {
            match dist.get(&next_state) {
                Some(&nd) if d + cost >= nd => {
                    continue;
                }
                _ => (),
            }
            dist.insert(next_state.clone(), d + cost);
            que.push((Reverse(d + cost), next_state));
        }
    }

    unreachable!();
}

type Coord = (i32, i32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct State {
    hall: [Cell; 7],
    rooms: [Room; 4],
}

impl State {
    fn new(rooms: [Room; 4]) -> Self {
        Self {
            hall: Default::default(),
            rooms,
        }
    }

    fn hall_amphis(&self) -> impl Iterator<Item = (usize, Amphi)> + '_ {
        self.hall
            .iter()
            .enumerate()
            .filter_map(|(id, cell)| Some((id, *cell.as_ref()?)))
    }

    fn room_amphis(&self) -> impl Iterator<Item = (usize, Amphi, i32)> + '_ {
        self.rooms.iter().enumerate().filter_map(|(id, room)| {
            let (y, amphi) = room.top()?;
            Some((id, amphi, y))
        })
    }

    fn vacant_between(&self, hall_id: usize, room_id: usize) -> bool {
        let room_right = room_id + 2;
        if hall_id < room_right {
            (hall_id + 1..room_right).all(|id| self.hall[id].is_none())
        } else {
            (room_right..hall_id).all(|id| self.hall[id].is_none())
        }
    }

    fn next(&self) -> impl Iterator<Item = (i64, Self)> + '_ {
        self.hall_amphis()
            .filter_map(|(hall_id, amphi)| {
                let target_room = amphi.target_room();
                if !self.vacant_between(hall_id, target_room) {
                    return None;
                }
                let (y, new_room) = self.rooms[target_room].push(amphi)?;
                let mut new_state = self.clone();
                new_state.rooms[target_room] = new_room;
                new_state.hall[hall_id] = None;
                let cost = distance((Room::x(target_room), y), hall_coord(hall_id)) * amphi.cost();
                Some((cost, new_state))
            })
            .chain(self.room_amphis().flat_map(move |(room_id, amphi, y)| {
                self.hall
                    .iter()
                    .enumerate()
                    .filter_map(move |(hall_id, cell)| {
                        if cell.is_some() || !self.vacant_between(hall_id, room_id) {
                            return None;
                        }
                        let mut new_state = self.clone();
                        new_state.hall[hall_id] = Some(amphi);
                        new_state.rooms[room_id].pop().unwrap();
                        let cost =
                            distance((Room::x(room_id), y), hall_coord(hall_id)) * amphi.cost();
                        Some((cost, new_state))
                    })
            }))
    }

    fn is_goal(&self) -> bool {
        use Amphi::*;
        use Room::*;
        self.rooms[0] == Two(A, A)
            && self.rooms[1] == Two(B, B)
            && self.rooms[2] == Two(C, C)
            && self.rooms[3] == Two(D, D)
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Room::*;
        for cell in &self.hall {
            match cell {
                None => write!(f, ".")?,
                Some(a) => write!(f, "{}", a)?,
            }
        }
        writeln!(f)?;
        write!(f, " ")?;
        for room in &self.rooms {
            match room {
                Two(t, _) => write!(f, "{}", t)?,
                _ => write!(f, ".")?,
            }
        }
        writeln!(f)?;
        write!(f, " ")?;
        for room in &self.rooms {
            match room {
                One(b) | Two(_, b) => write!(f, "{}", b)?,
                _ => write!(f, ".")?,
            }
        }
        Ok(())
    }
}

fn hall_coord(hall_id: usize) -> Coord {
    fn hall_x(hall_id: usize) -> i32 {
        [0, 1, 3, 5, 7, 9, 10][hall_id]
    }
    (hall_x(hall_id), 0)
}

type Cell = Option<Amphi>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Amphi {
    A,
    B,
    C,
    D,
}

impl Amphi {
    fn target_room(&self) -> usize {
        use Amphi::*;
        match self {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
        }
    }

    fn cost(&self) -> i64 {
        use Amphi::*;
        match self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }
}

impl TryFrom<char> for Amphi {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Amphi::*;
        Ok(match value {
            'A' => A,
            'B' => B,
            'C' => C,
            'D' => D,
            _ => bail!("invalid char"),
        })
    }
}

impl std::fmt::Display for Amphi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Amphi::*;
        let c = match self {
            A => 'A',
            B => 'B',
            C => 'C',
            D => 'D',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Room {
    Zero,
    One(Amphi),
    Two(Amphi, Amphi),
}

impl Room {
    fn top(&self) -> Option<(i32, Amphi)> {
        use Room::*;
        match self {
            Zero => None,
            &One(x) => Some((2, x)),
            &Two(x, _) => Some((1, x)),
        }
    }

    fn pop(&mut self) -> Option<Amphi> {
        use Room::*;
        match *self {
            Zero => None,
            One(a) => {
                *self = Zero;
                Some(a)
            }
            Two(a, b) => {
                *self = One(b);
                Some(a)
            }
        }
    }

    fn push(&self, amphi: Amphi) -> Option<(i32, Self)> {
        use Room::*;
        match self {
            Zero => Some((2, One(amphi))),
            &One(x) => Some((1, Two(amphi, x))),
            Two(_, _) => None,
        }
    }

    fn x(id: usize) -> i32 {
        id as i32 * 2 + 2
    }
}

fn distance(lhs: Coord, rhs: Coord) -> i64 {
    ((lhs.0 - rhs.0).abs() + (lhs.1 - rhs.1).abs()) as i64
}
