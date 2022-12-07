use std::collections::HashSet;

use aoc_utils::*;

fn find_marker<const N: usize>(data_stream: &str) -> usize {
    data_stream.as_bytes().windows(N).enumerate().find(|(_, window)| {
        window.iter().cloned().collect::<HashSet<u8>>().len() == N
    }).unwrap().0 + N
}

fn main() {
    let filename = input_filename();
    let input = read_lines(&filename);

    // Part 1
    let answer = find_marker::<4>(input.first().unwrap());
    println!("The answer to part one is: {:?}", answer);

    // Part 2
    let answer = find_marker::<14>(input.first().unwrap());
    println!("The answer to part two is: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_marker() {
        assert_eq!(find_marker::<4>("abcd"), 3);
    }
}
