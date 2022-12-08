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

fn scenic_score(grid: &Vec<Vec<usize>>, cell: (usize, usize)) -> usize {
    let (x, y) = cell;
    let height = grid[y][x];

    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;

    for i in (0..y).rev() {
        up += 1;
        if grid[i][x] >= height {
            break;
        }
    }

    for i in y + 1..grid.len() {
        down += 1;
        if grid[i][x] >= height {
            break;
        }
    }

    for i in (0..x).rev() {
        left += 1;
        if grid[y][i] >= height {
            break;
        }
    }

    for i in x + 1..grid[0].len() {
        right += 1;
        if grid[y][i] >= height {
            break;
        }
    }

    up * down * left * right
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
    println!("The answer to part one is: {:?}", answer);

    // Part 2
    let scenic_scores: Vec<Vec<usize>> = grid.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, _)| {
            scenic_score(&grid, (x, y))
        }).collect()
    }).collect();

    let answer = scenic_scores.iter().map(|row| row.iter().max().unwrap()).max().unwrap();
    println!("The answer to part two is: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;
}
    