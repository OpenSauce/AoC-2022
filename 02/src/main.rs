use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut total = 0;
    let mut total2 = 0;
    for line in contents.lines() {
        let ops: Vec<&str> = line.split(" ").collect();

        total += match_result(ops[0], ops[1]);
        total2 += match_result_two(ops[0], ops[1]);
    }

    println!("{}", total);
    println!("{}", total2);
}

fn match_result(x: &str, y: &str) -> i32 {
    let mut score = 0;
    match y {
        "X" => {
            score += 1;
            match x {
                "A" => score += 3,
                "B" => score += 0,
                "C" => score += 6,
                _ => panic!(),
            }
        }
        "Y" => {
            score += 2;
            match x {
                "A" => score += 6,
                "B" => score += 3,
                "C" => score += 0,
                _ => panic!(),
            }
        }
        "Z" => {
            score += 3;
            match x {
                "A" => score += 0,
                "B" => score += 6,
                "C" => score += 3,
                _ => panic!(),
            }
        }
        _ => panic!(),
    }

    return score;
}

fn match_result_two(x: &str, y: &str) -> i32 {
    let mut score = 0;
    match y {
        "X" => {
            score += 0;
            match x {
                "A" => score += 3,
                "B" => score += 1,
                "C" => score += 2,
                _ => panic!(),
            }
        }
        "Y" => {
            score += 3;
            match x {
                "A" => score += 1,
                "B" => score += 2,
                "C" => score += 3,
                _ => panic!(),
            }
        }
        "Z" => {
            score += 6;
            match x {
                "A" => score += 2,
                "B" => score += 3,
                "C" => score += 1,
                _ => panic!(),
            }
        }
        _ => panic!(),
    }

    return score;
}
