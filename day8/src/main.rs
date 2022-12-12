use std::cmp::max;
use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("Should be able to read input");

    let mut grid: Vec<Vec<u32>> = vec![];
    for line in input.lines() {
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        grid.push(digits);
    }

    let tallest_from_bottom = compute_tallest_from_bottom(&grid);
    let tallest_from_top = compute_tallest_from_top(&grid);
    let tallest_from_left = compute_tallest_from_left(&grid);
    let tallest_from_right = compute_tallest_from_right(&grid);

    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut num_visible = num_rows * 2 + num_cols * 2 - 4;

    for i in 1..(num_rows - 1) {
        for j in 1..(num_cols - 1) {
            if tallest_from_bottom[i][j] < grid[i][j]
                || tallest_from_top[i][j] < grid[i][j]
                || tallest_from_left[i][j] < grid[i][j]
                || tallest_from_right[i][j] < grid[i][j]
            {
                num_visible += 1;
            }
        }
    }
    println!("{}", num_visible);

    let mut max_scenic_score = 0;
    for i in 1..(num_rows - 1) {
        for j in 1..(num_cols - 1) {
            max_scenic_score = max(max_scenic_score, compute_scenic_score(&grid, i, j))
        }
    }
    println!("{}", max_scenic_score);
}

fn compute_scenic_score(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    let mut left_score = 0;
    for new_j in (0..j).rev() {
        left_score += 1;
        if grid[i][new_j] >= grid[i][j] {
            break;
        }
    }

    let mut right_score = 0;
    for new_j in (j + 1)..grid[0].len() {
        right_score += 1;
        if grid[i][new_j] >= grid[i][j] {
            break;
        }
    }

    let mut down_score = 0;
    for new_i in (i + 1)..grid.len() {
        down_score += 1;
        if grid[new_i][j] >= grid[i][j] {
            break;
        }
    }

    let mut up_score = 0;
    for new_i in (0..i).rev() {
        up_score += 1;
        if grid[new_i][j] >= grid[i][j] {
            break;
        }
    }

    left_score * right_score * down_score * up_score
}

fn compute_tallest_from_bottom(grid: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut result = grid.clone();

    for i in 1..(grid[0].len() - 1) {
        for j in (1..(grid.len() - 1)).rev() {
            result[j][i] = max(result[j + 1][i], grid[j + 1][i]);
        }
    }

    result
}

fn compute_tallest_from_top(grid: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut result = grid.clone();

    for i in 1..(grid[0].len() - 1) {
        for j in 1..(grid.len() - 1) {
            result[j][i] = max(result[j - 1][i], grid[j - 1][i]);
        }
    }

    result
}

fn compute_tallest_from_left(grid: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut result = grid.clone();

    for i in 1..(grid.len() - 1) {
        for j in 1..(grid[0].len() - 1) {
            result[i][j] = max(result[i][j - 1], grid[i][j - 1]);
        }
    }

    result
}

fn compute_tallest_from_right(grid: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut result = grid.clone();

    for i in 1..(grid.len() - 1) {
        for j in (1..(grid[0].len() - 1)).rev() {
            result[i][j] = max(result[i][j + 1], grid[i][j + 1]);
        }
    }

    result
}
