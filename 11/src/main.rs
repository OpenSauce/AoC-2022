#![feature(int_roundings)]

use itertools::Itertools;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut monkeys: Vec<Monkey> = Vec::new();

    for lines in &contents.lines().chunks(7) {
        let mut m = Monkey {
            inspections: 0,
            items: Vec::new(),
            op: Ops::times,
            op_amount: 0,
            test: 3,
            if_true: 0,
            if_false: 0,
        };

        let l = lines.collect::<Vec<&str>>();

        for i in l.get(1).unwrap().split(":").last().unwrap().split(",") {
            m.items.push(i.trim().parse().unwrap());
        }

        if l.get(2).unwrap().contains("old * old") {
            m.op = Ops::itself;
        } else if l.get(2).unwrap().contains("+") {
            m.op = Ops::plus;
            m.op_amount = l
                .get(2)
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
        } else {
            m.op = Ops::times;
            m.op_amount = l
                .get(2)
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
        }

        m.test = l
            .get(3)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        m.if_true = l
            .get(4)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        m.if_false = l
            .get(5)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        monkeys.push(m);
    }

    let lcm: usize = monkeys.iter().map(|m| m.test).product();
    for i in 0..10000 {
        let mut index = 0;
        for mut m in monkeys.clone() {
            while monkeys[index].items.len() > 0 {
                let mut v: usize = monkeys[index].items.pop().unwrap();
                match m.op {
                    Ops::itself => v *= v,
                    Ops::times => v *= m.op_amount,
                    Ops::plus => v += m.op_amount,
                }

                v = v % lcm;

                if v % m.test == 0 {
                    monkeys[m.if_true as usize].items.push(v);
                } else {
                    monkeys[m.if_false as usize].items.push(v);
                }

                monkeys[index].inspections += 1;
            }
            index += 1;
        }
    }

    for m in monkeys {
        println!("{}", m.inspections)
    }
}

#[derive(Clone)]
struct Monkey {
    inspections: usize,
    items: Vec<usize>,
    op: Ops,
    op_amount: usize,
    test: usize,
    if_true: usize,
    if_false: usize,
}

#[derive(Clone)]
enum Ops {
    times,
    plus,
    itself,
}
