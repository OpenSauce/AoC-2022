use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let cpy = contents.clone();

    exercise_one(contents);

    exercise_two(cpy);
}

fn exercise_one(contents: String) {
    let mut start = 0;
    let mut end = 4;
    while end < contents.len() {
        if all_unique(&contents[start..end]) {
            println!("{}", end);
            return;
        }
        start += 1;
        end += 1;
    }
}

fn exercise_two(contents: String) {
    let mut start = 0;
    let mut end = 14;
    while end < contents.len() {
        if all_unique(&contents[start..end]) {
            println!("{}", end);
            return;
        }
        start += 1;
        end += 1;
    }
}

fn all_unique(s: &str) -> bool {
    let mut seen = HashMap::<char, bool>::new();
    for c in s.chars() {
        if seen.contains_key(&c) {
            return false;
        }

        seen.insert(c, true);
    }
    return true;
}
