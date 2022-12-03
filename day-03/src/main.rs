use aoc_utils::*;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Eq, PartialEq)]
struct Bag {
    first: HashSet<char>,
    second: HashSet<char>,
}

impl Bag {
    fn error(&self) -> char {
        **self
            .first
            .intersection(&self.second)
            .into_iter()
            .peekable()
            .peek()
            .unwrap()
    }

    fn full_bag(&self) -> HashSet<&char> {
        self.first.union(&self.second).collect()
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

fn find_badge(team: &[Bag]) -> char {
    **team
        .iter()
        .map(|bag| bag.full_bag())
        .skip(1)
        .fold(team[0].full_bag(), |acc, next| {
            acc.intersection(&next).cloned().collect()
        })
        .iter()
        .next()
        .unwrap()
}

fn main() {
    let filename = input_filename();
    let input = read_lines(&filename);

    let bags = input.iter().map(|line| Bag::from_str(&line).unwrap());

    let priorities = bags.clone().map(|bag| bag.error().priority());

    // Part 1
    let answer = priorities.sum::<u32>();

    println!("The answer to part one is: {:?}", answer);

    // Part 2
    let binding = bags.collect::<Vec<Bag>>();
    let badges = binding
        .chunks(3)
        .map(|team| find_badge(team))
        .map(|badge| badge.priority());

    let answer = badges.sum::<u32>();

    println!("The answer to part two is: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;
}
