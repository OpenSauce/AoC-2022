use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    let mut max = [0, 0, 0];
    let mut current = 0;

    for line in lines {
        match line.parse::<i32>() {
            Ok(val) => current += val,
            Err(_) => {
                if current > max[2] {
                    max[2] = current;
                    max.sort();
                    max.reverse();
                }

                current = 0;
            }
        }
    }

    let sum: i32 = max.iter().sum();
    println!("Max: {}", sum);
}
