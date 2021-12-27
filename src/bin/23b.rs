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
    use Amphi::*;
    let mid = [(D, D), (C, B), (B, A), (A, C)];
    let rooms = top
        .zip(mid.into_iter())
        .zip(bottom)
        .map(|((t, (mt, mb)), b)| Room(vec![b, mb, mt, t]))
        .collect_vec()
        .try_into()
        .unwrap();
    State::new(rooms)
}

fn solve(input: &str) -> i64 {
    let init: State = parse(input);
    eprintln!("{}", &init);

    let mut dist = HashMap::<State, i64>::new();
    dist.insert(init.clone(), 0);
    let mut prev = HashMap::<State, State>::new();

    let mut que = BinaryHeap::new();
    que.push((Reverse(0), init));

    while let Some((Reverse(d), state)) = que.pop() {
        if state.is_goal() {
            let mut current = state;
            while let Some(s) = prev.get(&current) {
                current = s.clone();
                eprintln!("{}", current);
            }
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
            prev.insert(next_state.clone(), state.clone());
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
        self.rooms[0] == Room(vec![A; Room::DEPTH])
            && self.rooms[1] == Room(vec![B; Room::DEPTH])
            && self.rooms[2] == Room(vec![C; Room::DEPTH])
            && self.rooms[3] == Room(vec![D; Room::DEPTH])
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, cell) in self.hall.iter().enumerate() {
            if 1 < i && i < 6 {
                write!(f, " ")?;
            }
            match cell {
                None => write!(f, ".")?,
                Some(a) => write!(f, "{}", a)?,
            }
        }
        for height in (0..Room::DEPTH).rev() {
            writeln!(f)?;
            write!(f, " ")?;
            for Room(room) in &self.rooms {
                match room.get(height) {
                    Some(a) => write!(f, " {}", a)?,
                    _ => write!(f, " .")?,
                }
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Room(Vec<Amphi>);

impl Room {
    const DEPTH: usize = 4;

    fn top(&self) -> Option<(i32, Amphi)> {
        self.0
            .last()
            .map(|&amphi| (Self::DEPTH as i32 + 1 - self.0.len() as i32, amphi))
    }

    fn pop(&mut self) -> Option<Amphi> {
        self.0.pop()
    }

    fn push(&self, amphi: Amphi) -> Option<(i32, Self)> {
        if self.0.len() == Self::DEPTH || self.0.iter().any(|&a| a != amphi) {
            return None;
        }
        let mut room = self.clone();
        room.0.push(amphi);
        Some((Self::DEPTH as i32 - self.0.len() as i32, room))
    }

    fn x(id: usize) -> i32 {
        id as i32 * 2 + 2
    }
}

fn distance(lhs: Coord, rhs: Coord) -> i64 {
    ((lhs.0 - rhs.0).abs() + (lhs.1 - rhs.1).abs()) as i64
}
