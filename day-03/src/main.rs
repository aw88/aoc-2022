use std::{str::FromStr, collections::HashSet};
use aoc_utils::*;

#[derive(Debug, Eq, PartialEq)]
struct Bag {
    first: HashSet<char>,
    second: HashSet<char>,
}

impl Bag {
    fn error(&self) -> char {
        **self.first.intersection(&self.second).into_iter().peekable().peek().unwrap()
    }
}

impl FromStr for Bag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_at(s.len() / 2);

        Ok(Bag { 
            first: first.chars().collect(),
            second: second.chars().collect(),
        })
    } 
}

trait Priority {
    fn priority(&self) -> u32;
}

impl Priority for char {
    fn priority(&self) -> u32 {
        if self.is_uppercase() {
            *self as u32 - 38
        } else {
            *self as u32 - 96
        }
    }
}

fn main() {
    let filename = input_filename();
    let input = read_lines(&filename);

    let results = input.iter()
        .map(|line| Bag::from_str(&line).unwrap().error());

    let priorities = results.map(|c| c.priority());

    // Part 1
    let answer = priorities.sum::<u32>();
    
    println!("The answer to part one is: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;
}
