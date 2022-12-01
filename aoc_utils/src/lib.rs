use std::env;
use std::fs;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn input_filename() -> String {
    let default_file = "input.txt".to_string();
    env::args().collect::<Vec<String>>().get(1).unwrap_or(&default_file).clone()
}

pub fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Unable to read file")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
