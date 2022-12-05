use std::str::FromStr;

use aoc_utils::*;

#[derive(Debug)]
struct CraneState {
    stacks: [Vec<char>; 9],
}

impl CraneState {
    fn process_command(&self, command: &Command) -> CraneState {
        let mut new_stacks = self.stacks.clone();

        let mut from = self.stacks[command.from].clone();
        let mut to = self.stacks[command.to].clone();

        for _ in 0..command.count {
            to.push(from.pop().unwrap());
        }

        new_stacks[command.from] = from;
        new_stacks[command.to] = to;

        CraneState { stacks: new_stacks }
    }

    fn process_command2(&self, command: &Command) -> CraneState {
        let mut new_stacks = self.stacks.clone();

        let mut from = self.stacks[command.from].clone();
        let mut to = self.stacks[command.to].clone();
        let mut temp = vec![];

        for _ in 0..command.count {
            temp.push(from.pop().unwrap());
        }

        temp.iter().rev().for_each(|i| to.push(*i));

        new_stacks[command.from] = from;
        new_stacks[command.to] = to;

        CraneState { stacks: new_stacks }
    }
}

impl From<Vec<String>> for CraneState {
    fn from(mut v: Vec<String>) -> Self {
        v.pop();
        let rows = v.iter().rev();
        const EMPTY: Vec<char> = vec![];
        let mut stacks: [Vec<char>; 9] = [EMPTY; 9];

        rows.for_each(|row| {
            let row_chars = row.chars().collect::<Vec<char>>();
            let entries = row_chars.chunks(4);
            entries.enumerate().for_each(|(i, entry)| {
                if entry[1] != ' ' {
                    stacks[i].push(entry[1]);
                }
            });
        });

        CraneState { stacks }
    }
}

#[derive(Debug)]
struct Command {
    count: u32,
    from: usize,
    to: usize,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token: Vec<&str> = s.split(" ").collect();

        Ok(Command {
            count: u32::from_str(token[1]).unwrap(),
            from: usize::from_str(token[3]).unwrap() - 1,
            to: usize::from_str(token[5]).unwrap() - 1,
        })
    }
}

fn main() {
    let filename = input_filename();
    let input = read_lines(&filename);

    let start_state = input.iter().cloned().take_while(|line| line.len() > 0).collect::<Vec<String>>();
    let commands = input.iter().cloned().filter(|line| line.starts_with("move")).map(|c| Command::from_str(c.as_str()).unwrap());

    // Part 1
    let mut state = CraneState::from(start_state.clone());

    commands.clone().for_each(|command| {
        state = state.process_command(&command);
    });

    let answer = state.stacks.iter().map(|s| s.last()).filter(|c| c.is_some()).flatten().collect::<String>();

    println!("The answer to part one is: {:?}", answer);

    // Part 2
    let mut state = CraneState::from(start_state.clone());

    commands.clone().for_each(|command| {
        state = state.process_command2(&command);
    });

    let answer = state.stacks.iter().map(|s| s.last()).filter(|c| c.is_some()).flatten().collect::<String>();
    
    println!("The answer to part two is: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;
}
