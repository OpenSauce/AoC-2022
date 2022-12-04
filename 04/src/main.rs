use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut total = 0;
    let mut overlapping = 0;
    for line in contents.lines() {
        let packages: Vec<&str> = line.split(",").collect();
        if intersect(packages[0], packages[1]) {
            total += 1;
        }

        if overlap(packages[0], packages[1]) {
            overlapping += 1;
        }
    }

    println!("{} {}", total, overlapping);
}

fn intersect(x: &str, y: &str) -> bool {
    let left: Vec<i32> = x.split("-").map(|s| s.parse().unwrap()).collect();
    let right: Vec<i32> = y.split("-").map(|s| s.parse().unwrap()).collect();

    return (left[0] >= right[0] && left[1] <= right[1])
        || (right[0] >= left[0] && right[1] <= left[1]);
}

fn overlap(x: &str, y: &str) -> bool {
    let left: Vec<i32> = x.split("-").map(|s| s.parse().unwrap()).collect();
    let right: Vec<i32> = y.split("-").map(|s| s.parse().unwrap()).collect();

    return (left[0] >= right[0] && left[0] <= right[1]
        || left[1] <= right[1] && left[1] >= right[0])
        || (right[0] >= left[0] && right[0] <= left[1]
            || right[1] <= left[1] && right[1] >= left[0]);
}
