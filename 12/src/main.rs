use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        grid.push(line.chars().collect());
    }

    println!("{}", dfs(&mut grid, 0, 0, 0, 'E'));

    for row in grid {
        for c in row {
            print!("{}", c)
        }
        println!();
    }
}

fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize, c: usize, t: char) -> usize {
    if grid[i][j] == t {
        return c;
    }

    let mut up = usize::MAX;
    if i < grid.len() - 1
        && (gai(grid[i + 1][j]) - gai(grid[i][j]) == 1
            || gai(grid[i + 1][j]) - gai(grid[i][j]) == 0)
    {
        up = dfs(grid, i + 1, j, c + 1, t);
    }

    let mut down = usize::MAX;
    if i > 0
        && (gai(grid[i - 1][j]) - gai(grid[i][j]) == 1
            || gai(grid[i - 1][j]) - gai(grid[i][j]) == 0)
    {
        down = dfs(grid, i - 1, j, c + 1, t);
    }

    let mut right = usize::MAX;
    if j < grid[0].len() - 1
        && (gai(grid[i][j + 1]) - gai(grid[i][j]) == 1
            || gai(grid[i][j + 1]) - gai(grid[i][j]) == 0)
    {
        right = dfs(grid, i, j + 1, c + 1, t);
    }

    let mut left = usize::MAX;
    if j > 0
        && (gai(grid[i][j - 1]) - gai(grid[i][j]) == 1
            || gai(grid[i][j - 1]) - gai(grid[i][j]) == 0)
    {
        left = dfs(grid, i, j - 1, c + 1, t);
    }

    return right.min(left.min(up.min(down)));
}

fn gai(c: char) -> i32 {
    if (c as i32) < 97 {
        return (c as i32) - 38;
    }

    return (c as i32) - 96;
}
