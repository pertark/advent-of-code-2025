#![allow(unused)]
use std::fs;

static DAY: u8 = 1;
type Input = Vec<String>;

fn read_input(file: &str) -> Input {
    let input = fs::read_to_string(format!("data/{}/{}", DAY, file)).expect("Failed to read input file");
    println!("==== {} ====", file);
    input.lines().map(|line| line.to_string()).collect()
}

fn part_one(input: Input) {
    println!("{:?}", input);
    unimplemented!()
}

fn part_two(input: Input) {
    unimplemented!()
}

fn main() {
    let input = read_input("example");
    part_one(input.clone());

    let input = read_input("input");
    part_one(input.clone());

    // let input = read_input("example");
    // part_two(input.clone());

    // let input = read_input("input");
    // part_two(input.clone());
}