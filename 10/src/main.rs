use std::env;
use std::fs;
use std::io::Lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut ops: Vec<i32> = Vec::new();

    for line in contents.lines() {
        if line.starts_with("noop") {
            ops.push(0);
        }

        if line.starts_with("addx") {
            ops.push(0);
            ops.push(line.split(" ").nth(1).unwrap().parse().unwrap());
        }
    }

    let mut x = 1;
    let mut cycle = 0;
    let mut sum = 0;
    for op in ops.clone() {
        if cycle % 40 == 0 {
            println!();
        }

        cycle += 1;
        if (cycle - 20) % 40 == 0 {
            sum += cycle * x;
        }

        if cycle % 40 >= x && cycle % 40 <= x + 2 {
            print!("#");
        } else {
            print!(".");
        }

        x += op;
    }

    println!("{}", sum);
}
