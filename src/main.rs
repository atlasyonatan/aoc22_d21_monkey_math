use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

mod monkey_math;
use crate::monkey_math::{operator::Operator, parse};

fn main() {
    let file_path = "../input.txt";
    let path = Path::new(file_path);
    let file = File::open(path).unwrap();

    let mut operators = HashMap::new();
    operators.insert("+".to_string(), Operator::new(|a, b| a + b));
    operators.insert("-".to_string(), Operator::new(|a, b| a - b));
    operators.insert("*".to_string(), Operator::new(|a, b| a * b));
    operators.insert("/".to_string(), Operator::new(|a, b| a / b));

    let monkeys: HashMap<_, _> = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|l| parse(&l))
        .map(Option::unwrap)
        .collect();
    let result: usize = monkeys
        .get("root")
        .unwrap()
        .evaluate(&monkeys, &operators)
        .unwrap();
    println!("part 1: {}", result);
}
