use std::fs;

static DAY: u8 = 1;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    clicks: i32,
}

fn read_input() -> Vec<Instruction> {
    let input = fs::read_to_string(format!("data/{}/input", DAY)).expect("Failed to read input file");
    input.lines().map(|line| {
        let direction = match line.chars().next().unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction"),
        };
        let clicks = line[1..].parse().unwrap();
        Instruction { direction, clicks }
    }).collect()
}

fn read_example() -> Vec<Instruction> {
    let input = fs::read_to_string(format!("data/{}/example", DAY)).expect("Failed to read input file");
    input.lines().map(|line| {
        let direction = match line.chars().next().unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction"),
        };
        let clicks = line[1..].parse().unwrap();
        Instruction { direction, clicks }
    }).collect()
}

fn part_one(input: Vec<Instruction>) {
    let mut state = 50;
    let mut zeroes = 0;
    for instr in input {
        match instr.direction {
            Direction::Left => state -= instr.clicks,
            Direction::Right => state += instr.clicks,
        }

        state = ((state % 100) + 100) % 100;
        // println!("State: {}", state);

        if state == 0 {
            zeroes += 1;
        }
    }
    println!("Part one: {}", zeroes);
}

fn part_two(input: Vec<Instruction>) {
    let mut state = 50;
    let mut zeroes = 0;
    for instr in input {
        // who wants to do it smart when the computer can do it
        let inc = match instr.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        for _ in 0..instr.clicks {
            state += inc;
            state = ((state % 100) + 100) % 100;
            if state == 0 {
                zeroes += 1;
            }
        }
    }
    println!("Part two: {}", zeroes);
}

fn main() {
    let example = read_example();
    part_one(example.clone());
    part_two(example.clone());
    let input = read_input();
    part_one(input.clone());
    part_two(input.clone());
}