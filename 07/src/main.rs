use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    calculate(contents.as_str())
}

fn calculate(s: &str) {
    let mut stack: Vec<&str> = Vec::new();
    let mut seen = HashMap::<String, i32>::new();

    for l in s.lines() {
        if l.starts_with("$") {
            let cmd: Vec<&str> = l.split(" ").collect();
            if cmd[1] == "cd" {
                if cmd[2] == ".." {
                    stack.pop();
                    continue;
                }

                stack.push(cmd[2])
            }
            continue;
        }

        if !l.starts_with("dir") {
            for (i, _) in stack.iter().enumerate() {
                let dir = (&stack)
                    .iter()
                    .copied()
                    .take(i)
                    .collect::<Vec<_>>()
                    .join("");

                let size: i32 = l.split(" ").next().unwrap().parse().unwrap();
                if !seen.contains_key(&dir) {
                    seen.insert(dir, size);
                    continue;
                }

                *seen.get_mut(&dir).unwrap() += size;
            }
        }
    }

    let mut sum = 0;
    let mut lowest = i32::MAX;
    let needed = 30000000 - (70000000 - seen.get("/").unwrap());
    for v in seen {
        if v.1 <= 100000 {
            sum += v.1;
        }

        if needed <= v.1 && v.1 < lowest {
            lowest = v.1;
        }
    }

    println!("{} {}", sum, lowest)
}
