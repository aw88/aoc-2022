use std::{collections::HashSet, str::FromStr};

use aoc_utils::*;

#[derive(Debug, PartialEq, Eq)]
struct Assignment(HashSet<i32>);

impl FromStr for Assignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once("-").unwrap();
        let (from, to) = (i32::from_str(from).unwrap(), i32::from_str(to).unwrap());

        Ok(Assignment((from..(to + 1)).collect::<HashSet<i32>>()))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SectionAssignment {
    first: Assignment,
    second: Assignment,
}

impl SectionAssignment {
    fn fully_contains(&self) -> bool {
        self.first.0.is_subset(&self.second.0) || self.second.0.is_subset(&self.first.0)
    }

    fn overlaps(&self) -> bool {
        self.first.0.intersection(&self.second.0).count() > 0
    }
}

impl FromStr for SectionAssignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((first, second)) = s.split_once(",") {
            Ok(SectionAssignment {
                first: Assignment::from_str(first).unwrap(),
                second: Assignment::from_str(second).unwrap(),
            })
        } else {
            Err(())
        }
    }
}

fn main() {
    let filename = input_filename();
    let input = read_lines(&filename);

    let section_assignments = input
        .iter()
        .map(|line| SectionAssignment::from_str(line).unwrap());

    // Part 1
    let fully_overlapping_assignments = section_assignments.clone().filter(|a| a.fully_contains());

    println!(
        "The answer to part one is: {:?}",
        fully_overlapping_assignments.count()
    );

    // Part 2
    let overlapping_assignments = section_assignments.clone().filter(|a| a.overlaps());

    println!(
        "The answer to part two is: {:?}",
        overlapping_assignments.count()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignment_parse() {
        let cases = vec![
            ("1-1", vec![1]),
            ("4-5", vec![4, 5]),
            ("1-3", vec![1, 2, 3]),
        ];

        for (input, expected_result) in cases.iter() {
            let assignment = Assignment::from_str(input).unwrap();
            let expected_result = Assignment(expected_result.iter().cloned().collect());

            assert_eq!(assignment, expected_result);
        }
    }

    #[test]
    fn test_overlapping_assignments() {
        let cases = vec![
            (vec![1], vec![3, 4, 5], false),
            (vec![2, 3], vec![1, 2, 3, 4, 5], true),
        ];

        for (first, second, expected_result) in cases.iter() {
            let section_assignment = SectionAssignment {
                first: Assignment(first.iter().cloned().collect()),
                second: Assignment(second.iter().cloned().collect()),
            };

            assert_eq!(section_assignment.fully_contains(), *expected_result);
        }
    }
}
