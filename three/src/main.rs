use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.lines();
    let mut total = 0;
    for line in lines {
        let packages = line.split_at(line.len() / 2);
        total += get_priority(packages.0, packages.1)
    }

    println!("{}", total);

    let lines: Vec<&str> = contents.lines().collect();
    let mut total = 0;
    let mut i = 0;
    while i < lines.len() - 2 {
        total += get_badges(lines[i], lines[i + 1], lines[i + 2]);
        i += 3
    }

    println!("{}", total);
}

fn get_badges(a: &str, b: &str, c: &str) -> u32 {
    let mut seen1 = HashMap::<char, bool>::new();
    let mut seen2 = HashMap::<char, bool>::new();

    for c in a.chars() {
        seen1.insert(c, true);
    }

    for c in b.chars() {
        seen2.insert(c, true);
    }

    for c in c.chars() {
        if seen1.contains_key(&c) && seen2.contains_key(&c) {
            return get_alhabet_index(c);
        }
    }

    return 0;
}

fn get_priority(a: &str, b: &str) -> u32 {
    let mut seen = HashMap::<char, bool>::new();

    for c in a.chars() {
        seen.insert(c, true);
    }

    for c in b.chars() {
        if seen.contains_key(&c) {
            return get_alhabet_index(c);
        }
    }

    return 0;
}

fn get_alhabet_index(c: char) -> u32 {
    if (c as u32) < 97 {
        return (c as u32) - 38;
    }

    return (c as u32) - 96;
}
