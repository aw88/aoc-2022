use std::collections::HashSet;

use aoc_utils::*;

fn transpose(v: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut transposed: Vec<Vec<usize>> = Vec::new();

    for i in 0..v.len() {
        transposed.push(Vec::new());

        for j in 0..v[0].len() {
            transposed[i].push(v[j][i]);
        }
    }

    transposed
}

fn is_visible(row: &Vec<usize>, index: usize) -> bool {
    let height = row[index];

    let visible_from_left = row[0..index].iter().all(|&t| t < height);
    let visible_from_right = row[(index+1)..(row.len())].iter().all(|&t| t < height);

    visible_from_left | visible_from_right
}

fn main() {
    let filename = input_filename();
    let input = read_lines(&filename);

    let grid: Vec<Vec<usize>> = input.iter().map(|line| line.split("").flat_map(|c| c.parse::<usize>()).collect()).collect();

    // Part 1
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if is_visible(row, j) {
                visible_trees.insert((i, j));
            }
        }
    }

    for (i, row) in transpose(&grid).iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if is_visible(row, j) {
                visible_trees.insert((j, i));
            }
        }
    }

    let answer = visible_trees.len();
    println!("The solution to part one is: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;
}
    