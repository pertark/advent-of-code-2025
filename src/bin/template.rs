use std::fs;

static DAY: u8 = 1;

fn read_input() -> Vec<String> {
    let input = fs::read_to_string(format!("data/{}/input", DAY)).expect("Failed to read input file");
    input.lines().map(|line| line.to_string()).collect()
}

fn part_one(input: Vec<String>) {
    println!("{:?}", input);
    unimplemented!()
}

fn part_two(input: Vec<String>) {
    unimplemented!()
}

fn main() {
    let input = read_input();
    part_one(input.clone());
    part_two(input.clone());
}