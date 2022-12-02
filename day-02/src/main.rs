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
struct Game {
    first: Hand,
    second: Hand,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s.split(" ")
            .map(|h| Hand::from_str(h).unwrap())
            .take(2)
            .collect::<Vec<Hand>>();
        
        Ok(Game {
            first: hands[0],
            second: hands[1],
        })
    }
}

impl Game {
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(()),
        }
    }
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

#[derive(Debug)]
struct Suggestion {
    first: Hand,
    outcome: GameResult,
}

impl Suggestion {
    fn suggestion(&self) -> Hand {
        match (self.first, self.outcome) {
            (Hand::Rock, GameResult::Win) => Hand::Paper,
            (Hand::Rock, GameResult::Draw) => Hand::Rock,
            (Hand::Rock, GameResult::Lose) => Hand::Scissors,
            (Hand::Paper, GameResult::Win) => Hand::Scissors,
            (Hand::Paper, GameResult::Draw) => Hand::Paper,
            (Hand::Paper, GameResult::Lose) => Hand::Rock,
            (Hand::Scissors, GameResult::Win) => Hand::Rock,
            (Hand::Scissors, GameResult::Draw) => Hand::Scissors,
            (Hand::Scissors, GameResult::Lose) => Hand::Paper,
        }
    }
}

impl FromStr for Suggestion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let suggestion = s.split(" ")
            .take(2)
            .collect::<Vec<&str>>();
        
        Ok(Suggestion {
            first: Hand::from_str(suggestion[0]).unwrap(),
            outcome: GameResult::from_str(suggestion[1]).unwrap(),
        })
    }
}

fn main() {
    let filename = input_filename();
    let input = read_lines(&filename);

    let results = input.iter()
        .map(|l| Game::from_str(l).unwrap().score());

    // Part 1
    let answer = results.sum::<i32>();
    
    println!("The answer to part one is: {:?}", answer);

    // Part 2
    let results = input.iter()
        .map(|l| {
            let suggestion = Suggestion::from_str(l).unwrap();
            Game {
                first: suggestion.first,
                second: suggestion.suggestion(),
            }
        });

    let answer = results.map(|f| f.score()).sum::<i32>();

    println!("The answer to part two is: {:?}", answer);
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
        let cases: Vec<(&str, Result<Game, ()>)> = vec![
            ("A Y", Ok(Game { first: Hand::Rock, second: Hand::Paper })),
        ];

        for (game, result) in cases.iter() {
            let parsed_game = Game::from_str(game);
            assert_eq!(&parsed_game, result);
        }
    }
}
