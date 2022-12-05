use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let num_of_stacks: usize = args[2].parse().unwrap();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut stacks = Vec::new();
    for _ in 0..num_of_stacks {
        let stack: Vec<char> = Vec::new();
        stacks.push(stack)
    }

    for line in contents.lines() {
        if line.starts_with(" 1   2   3 ") {
            break;
        }

        for i in 0usize..num_of_stacks {
            let c = line.chars().nth(1 + (i * 4)).unwrap();
            if c.is_alphabetic() {
                stacks[i].insert(0, c);
            }
        }
    }

    let mut new_stacks = stacks.to_vec();

    for line in contents.lines() {
        if !line.starts_with("move") {
            continue;
        }

        let m: Vec<usize> = line.split(" ").filter_map(|s| s.parse().ok()).collect();

        for _ in 0usize..m[0] {
            let elem = stacks[m[1] - 1].pop().unwrap();
            stacks[m[2] - 1].push(elem);
        }

        let i = new_stacks[m[1] - 1].len() - m[0];
        let mut elems = new_stacks[m[1] - 1].split_off(i);
        new_stacks[m[2] - 1].append(&mut elems)
    }

    for mut stack in stacks {
        print!("{}", stack.pop().unwrap());
    }

    println!("");

    for mut stack in new_stacks {
        print!("{}", stack.pop().unwrap());
    }
}
