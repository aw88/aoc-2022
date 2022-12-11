use std::{str::FromStr, iter::Cycle};

use aoc_utils::*;

#[derive(Debug)]
enum Command {
    NoOp,
    AddX(i32),
}

impl Command {
    fn cycles(&self) -> usize {
        match self {
            Command::NoOp => 1,
            Command::AddX(_) => 2,
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(' ').collect::<Vec<&str>>();

        match tokens[0] {
            "noop" => Ok(Command::NoOp),
            "addx" => Ok(Command::AddX(tokens[1].parse().unwrap())),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct State {
    cycles: usize,
    reg_x: i32,
}

fn process_command(state: &State, command: Command) -> State {
    let mut cycles = state.cycles;
    let mut reg_x = state.reg_x;

    match command {
        Command::NoOp => (),
        Command::AddX(x) => reg_x = reg_x + x,
    }

    cycles = cycles + command.cycles();

    State { cycles, reg_x }
}

fn signal_strength(state_history: &Vec<State>, cycle: usize) -> i32 {
    for i in 0..state_history.len()-2 {
        if state_history[i+1].cycles >= cycle {
            return cycle as i32 * state_history[i].reg_x;
        }
    }
    0
}

fn main() {
    let filename = input_filename();
    let input = read_lines(&filename);

    let commands: Vec<Command> = input.iter().map(|c| c.parse().unwrap()).collect();

    let mut state = State {
        cycles: 0,
        reg_x: 1,
    };

    let mut state_history = vec![state];
    for command in commands {
        state = process_command(&state, command);
        state_history.push(state);
    }

    for state in state_history.iter() {
        println!("{:?}", state);
    }

    let cycles = [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|&c| signal_strength(&state_history, c));
    
    let answer = cycles.sum::<i32>();
    println!("The answer to part one is: {:?}", answer);
}
