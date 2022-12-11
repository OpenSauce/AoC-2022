#![feature(int_roundings)]

use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut seen: HashMap<String, bool> = HashMap::new();

    // initial state
    let mut head = Node { x: 0, y: 0 };
    let mut tail = Node { x: 0, y: 0 };

    seen.insert(tail.x.to_string() + ":" + &tail.y.to_string(), true);

    for line in contents.lines() {
        let ops: Vec<&str> = line.split(" ").collect();
        for _ in 0..ops[1].parse().unwrap() {
            move_head(&mut head, ops[0]);
            move_tail(&head, &mut tail);
            seen.insert(tail.x.to_string() + ":" + &tail.y.to_string(), true);
        }
    }

    println!("{}", seen.iter().count());

    let mut knots = vec![Node::new(); 10];

    let mut seen2: HashMap<String, bool> = HashMap::new();

    for line in contents.lines() {
        let ops: Vec<&str> = line.split(" ").collect();
        for _ in 0..ops[1].parse().unwrap() {
            move_head(&mut knots[0], ops[0]);
            for i in 1..10 {
                let cpy = (&mut knots[i - 1]).clone();
                move_tail(&cpy, &mut knots[i]);
            }
            seen2.insert(knots[9].x.to_string() + ":" + &knots[9].y.to_string(), true);
        }
    }

    println!("{}", seen2.iter().count());
}

fn move_head(head: &mut Node, dir: &str) {
    match dir {
        "R" => head.x += 1,
        "L" => head.x -= 1,
        "U" => head.y += 1,
        "D" => head.y -= 1,
        _ => panic!(),
    }
}

fn move_tail(head: &Node, tail: &mut Node) {
    let dx = head.x - tail.x;
    let dy = head.y - tail.y;

    if dx.abs() <= 1 && dy.abs() <= 1 {
        return;
    }

    if dx.abs() > 1 && dy.abs() == 0 {
        tail.x += dx.div_floor(dx.abs());
    }

    if dx.abs() == 0 && dy.abs() > 1 {
        tail.y += dy.div_floor(dy.abs());
    }

    if dx.abs() > 0 && dy.abs() > 0 {
        tail.x += dx.div_floor(dx.abs());
        tail.y += dy.div_floor(dy.abs());
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
struct Node {
    x: i32,
    y: i32,
}

impl Node {
    pub fn new() -> Self {
        Node { x: 0, y: 0 }
    }
}
