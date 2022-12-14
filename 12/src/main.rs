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

    println!("{}", bfs(&mut grid.clone(), 0, 20));

    let mut lowest = usize::MAX;
    for i in 0..grid.len() - 1 {
        for j in 0..grid[0].len() - 1 {
            if grid[i][j] == 'a' || grid[i][j] == 'S' {
                let value = bfs(&mut grid.clone(), j, i);
                if value < lowest {
                    lowest = value;
                }
            }
        }
    }

    println!("{}", lowest);
}

fn bfs(grid: &mut Vec<Vec<char>>, startx: usize, starty: usize) -> usize {
    let mut queue: Vec<Point> = Vec::new();
    let mut count = 0;

    queue.push(Point {
        x: startx,
        y: starty,
    });
    while queue.len() > 0 {
        let len = queue.len();
        for _ in 0..len {
            let v = queue.remove(0);
            let i = v.y;
            let j = v.x;

            if grid[i][j] == 'E' {
                return count;
            }

            if grid[i][j] == '.' {
                continue;
            }

            if i < grid.len() - 1 && (gai(grid[i + 1][j]) - gai(grid[i][j]) <= 1) {
                queue.push(Point { x: v.x, y: v.y + 1 })
            }

            if i > 0 && (gai(grid[i - 1][j]) - gai(grid[i][j]) <= 1) {
                queue.push(Point { x: v.x, y: v.y - 1 })
            }

            if j < grid[0].len() - 1 && (gai(grid[i][j + 1]) - gai(grid[i][j]) <= 1) {
                queue.push(Point { x: v.x + 1, y: v.y })
            }

            if j > 0 && (gai(grid[i][j - 1]) - gai(grid[i][j]) <= 1) {
                queue.push(Point { x: v.x - 1, y: v.y })
            }

            grid[i][j] = '.';
        }

        count += 1;
    }

    return usize::MAX;
}

struct Point {
    x: usize,
    y: usize,
}

fn gai(c: char) -> i32 {
    if c == 'E' {
        return gai('z');
    }
    if c == 'S' {
        return gai('a');
    }
    if (c as i32) < 97 {
        return (c as i32) - 38;
    }

    return (c as i32) - 96;
}
