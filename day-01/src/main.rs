use aoc_utils::*;

fn main() {
    let filename = input_filename();
    let input = read_lines(&filename);
    
    let gnome_calories = input.split(|line| line == "")
        .map(|gnome| {
            gnome.iter().map(|calories| {
                calories.parse::<i32>().unwrap()
            })
            .sum::<i32>()
        });

    // Part 1
    let answer = gnome_calories.clone().max().unwrap();

    println!("The answer to part one is: {:?}", answer);

    // Part 2
    let mut sorted_gnome_calories = gnome_calories.clone().collect::<Vec<i32>>();
    sorted_gnome_calories.sort();

    let answer = sorted_gnome_calories.iter().rev().take(3).sum::<i32>();

    println!("The answer to part two is: {:?}", answer);
}
