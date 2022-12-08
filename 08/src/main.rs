use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let max_x = contents.lines().nth(0).unwrap().chars().count();
    let max_y = contents.lines().count();

    let mut grid = vec![vec![0; max_y]; max_x];
    let mut seen = vec![vec![false; max_y]; max_x];

    // Populate initial state.
    for (i, line) in contents.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let n = c.to_digit(10).unwrap();
            grid[i][j] = n;
        }
    }

    let mut count = max_y * 2 + max_x * 2 - 4;
    let mut i = 1;
    while i < grid.len() - 1 {
        let mut left = 1;
        let mut l_highest = grid[i][0];
        while left < grid[i].len() - 2 {
            if grid[i][left] > l_highest {
                if !seen[i][left] {
                    count += 1;
                    seen[i][left] = true;
                }

                l_highest = grid[i][left];
            }

            left += 1;
        }
        i += 1;
    }

    let mut i = 1;
    while i < grid.len() - 1 {
        let mut right = grid[i].len() - 1;
        let mut r_highest = grid[i][grid[i].len() - 1];
        while 1 <= right {
            if grid[i][right] > r_highest {
                if !seen[i][right] {
                    count += 1;
                    seen[i][right] = true;
                }
                r_highest = grid[i][right];
            }

            right -= 1;
        }
        i += 1;
    }

    let mut i = 1;
    while i < grid.len() - 1 {
        let mut top = 1;
        let mut l_highest = grid[0][i];
        while top <= grid.len() - 2 {
            if grid[top][i] > l_highest {
                if !seen[top][i] {
                    count += 1;
                    seen[top][i] = true;
                }

                l_highest = grid[top][i];
            }

            top += 1;
        }
        i += 1;
    }

    let mut i = 1;
    while i < grid.len() - 1 {
        let mut bottom = grid.len() - 2;
        let mut r_highest = grid[grid.len() - 1][i];
        while 1 <= bottom {
            if grid[bottom][i] > r_highest {
                if !seen[bottom][i] {
                    count += 1;
                    seen[bottom][i] = true;
                }
                r_highest = grid[bottom][i];
            }
            bottom -= 1;
        }
        i += 1;
    }

    println!("{}", count);

    let mut max_score = 0;

    let mut i = 1;
    while i < grid.len() - 1 {
        let mut j = 1;
        while j < grid[i].len() - 1 {
            let mut score: Vec<i32> = Vec::new();

            let mut x = i - 1;
            let mut y = j;
            let mut running_score = 0;
            while x > 0 {
                if grid[x][y] < grid[i][j] {
                    running_score += 1;
                }
                if grid[x][y] >= grid[i][j] {
                    running_score += 1;
                    break;
                }

                if x == 1 {
                    if grid[0][y] < grid[i][j] {
                        running_score += 1;
                    }
                }

                x -= 1;
            }
            score.push(running_score);

            let mut x = i;
            let mut y = j - 1;
            let mut running_score = 0;
            while y > 0 {
                if grid[x][y] < grid[i][j] {
                    running_score += 1;
                }
                if grid[x][y] >= grid[i][j] {
                    running_score += 1;
                    break;
                }

                if y == 1 {
                    if grid[x][0] < grid[i][j] {
                        running_score += 1;
                    }
                }

                y -= 1;
            }
            score.push(running_score);

            let mut x = i + 1;
            let mut y = j;
            let mut running_score = 0;
            while x < grid.len() {
                if grid[x][y] < grid[i][j] {
                    running_score += 1;
                }
                if grid[x][y] >= grid[i][j] {
                    running_score += 1;
                    break;
                }

                x += 1;
            }
            score.push(running_score);

            let mut x = i;
            let mut y = j + 1;
            let mut running_score = 0;
            while y < grid[i].len() {
                if grid[x][y] < grid[i][j] {
                    running_score += 1;
                }
                if grid[x][y] >= grid[i][j] {
                    running_score += 1;
                    break;
                }

                y += 1;
            }
            score.push(running_score);

            let mut scene_score = 1;
            for val in score {
                if val > 0 {
                    scene_score *= val;
                }
            }

            if scene_score > max_score {
                max_score = scene_score;
            }

            j += 1;
        }
        i += 1;
    }

    println!("{}", max_score)
}
