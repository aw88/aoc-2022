use std::str::FromStr;
use aoc_utils::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn score(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A"|"X" => Ok(Hand::Rock),
            "B"|"Y" => Ok(Hand::Paper),
            "C"|"Z" => Ok(Hand::Scissors),
            _ => Err(()),
        }        
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct RPSGame {
    first: Hand,
    second: Hand,
}

impl FromStr for RPSGame {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s.split(" ")
            .map(|h| Hand::from_str(h).unwrap())
            .take(2)
            .collect::<Vec<Hand>>();
        
        Ok(RPSGame {
            first: hands[0],
            second: hands[1],
        })
    }
}

impl RPSGame {
    fn result(&self) -> GameResult {
        match (self.first, self.second) {
            (Hand::Rock, Hand::Rock) => GameResult::Draw,
            (Hand::Rock, Hand::Paper) => GameResult::Win,
            (Hand::Rock, Hand::Scissors) => GameResult::Lose,
            (Hand::Paper, Hand::Rock) => GameResult::Lose,
            (Hand::Paper, Hand::Paper) => GameResult::Draw,
            (Hand::Paper, Hand::Scissors) => GameResult::Win,
            (Hand::Scissors, Hand::Rock) => GameResult::Win,
            (Hand::Scissors, Hand::Paper) => GameResult::Lose,
            (Hand::Scissors, Hand::Scissors) => GameResult::Draw,
        }
    }

    fn score(&self) -> i32 {
        self.result().score() + self.second.score()
    }
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    fn score(&self) -> i32 {
        match self {
            GameResult::Win => 6,
            GameResult::Lose => 0,
            GameResult::Draw => 3,
        }
    }
}

fn main() {
    let filename = input_filename();
    let input = read_lines(&filename);

    let results = input.iter()
        .map(|l| RPSGame::from_str(l).unwrap().score());

    // Part 1
    let answer = results.sum::<i32>();
    
    println!("The answer to part one is: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hands() {
        let cases: Vec<(&str, Result<Hand, ()>)> = vec![
            ("X", Ok(Hand::Rock)),
            ("Y", Ok(Hand::Paper)),
            ("Z", Ok(Hand::Scissors)),
            ("A", Ok(Hand::Rock)),
            ("B", Ok(Hand::Paper)),
            ("C", Ok(Hand::Scissors)),
            ("D", Err(())),
            ("E", Err(())),
            ("F", Err(())),
        ];

        for (hand, result) in cases.iter() {
            let parsed_hand = Hand::from_str(hand);
            assert_eq!(&parsed_hand, result);
        }
    }

    #[test]
    fn parse_games() {
        let cases: Vec<(&str, Result<RPSGame, ()>)> = vec![
            ("A Y", Ok(RPSGame { first: Hand::Rock, second: Hand::Paper })),
        ];

        for (game, result) in cases.iter() {
            let parsed_game = RPSGame::from_str(game);
            assert_eq!(&parsed_game, result);
        }
    }
}
