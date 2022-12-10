use std::{str::FromStr, collections::HashSet};

use aoc_utils::*;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('U') => Ok(Direction::Up),
            Some('D') => Ok(Direction::Down),
            Some('L') => Ok(Direction::Left),
            Some('R') => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    count: usize,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((direction, count)) = s.split_once(' ') {
            Ok(Command {
                direction: direction.parse().unwrap(),
                count: count.parse().unwrap(),
            })
        } else {
            Err(())
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    head: (i32, i32),
    tail: (i32, i32),
}

impl State {
    fn new(head: (i32, i32), tail: (i32, i32)) -> State {
        State { head, tail }
    }

    fn next(&self, direction: &Direction) -> State {
        let (dx, dy) = direction.delta();
        let (mut hx, mut hy) = self.head;
        let (mut tx, mut ty) = self.tail;

        hx = hx + dx;
        hy = hy + dy;

        if (i32::abs(tx - hx) <= 1) && (i32::abs(ty - hy) <= 1) {

        } else {
            if hx == tx || hy == ty {
                tx = tx + dx;
                ty = ty + dy;
            } else {
                tx = self.head.0;
                ty = self.head.1;
            }
        }
        // Same row or column
        // if hx == tx || hy == ty {
        //     if (i32::abs(tx - hx) > 1) ^ (i32::abs(ty - hy) > 1) {
        //         tx = tx + dx;
        //         ty = ty + dy;
        //     }
        // } else {
        //     tx = self.head.0;
        //     ty = self.head.1;
        // }

        State {
            head: (hx, hy),
            tail: (tx, ty),
        }
    }
}

fn main() {
    let filename = input_filename();
    let input = read_lines(&filename);

    let commands: Vec<Command> = input.iter().map(|line| line.parse().unwrap()).collect();

    let mut state = State::new((0, 0), (0, 0));

    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    tail_positions.insert((0, 0));

    for command in commands {
        for _ in 0..command.count {
            state = state.next(&command.direction);
            tail_positions.insert(state.tail);
        }
    }

    let answer = tail_positions.len();
    println!("The answer to part one is: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let cases = vec![
            // ... => ...
            // X.. => TH.
            (State::new((0, 0), (0, 0)), Direction::Right, State::new((1, 0), (0, 0))),
            // ... => ...
            // TH. => .TH
            (State::new((1, 0), (0, 0)), Direction::Right, State::new((2, 0), (1, 0))),
            // .H. => .TH
            // T.. => ...
            (State::new((1, 1), (0, 0)), Direction::Right, State::new((2, 1), (1, 1))),
            // H. => .H
            // T. => T.
            (State::new((0, 1), (0, 0)), Direction::Right, State::new((1, 1), (0, 0))),


            // ... => ...
            // .X. => HT.
            (State::new((1, 0), (1, 0)), Direction::Left, State::new((0, 0), (1, 0))),
            // ... => ...
            // .HT => HT.
            (State::new((1, 0), (2, 0)), Direction::Left, State::new((0, 0), (1, 0))),
            // .H. => HT.
            // ..T => ...
            (State::new((1, 1), (2, 0)), Direction::Left, State::new((0, 1), (1, 1))),
        ];

        for (state, direction, expected) in cases {
            let next = state.next(&direction);

            assert_eq!(next, expected, "Moving {:?}", direction);
        }
    }
}
