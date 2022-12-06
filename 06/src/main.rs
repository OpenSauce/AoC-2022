use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", calculate(contents.as_str(), 4));
    println!("{}", calculate(contents.as_str(), 14));
}

fn calculate(contents: &str, n: usize) -> usize {
    let mut start = 0;
    let mut end = n;
    while end < contents.len() {
        if all_unique(&contents[start..end]) {
            return end;
        }
        start += 1;
        end += 1;
    }
    return 0;
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
