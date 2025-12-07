#![allow(unused)]
use std::fs;

static DAY: u8 = 7;
type Input = Vec<Vec<char>>;

fn read_input(file: &str) -> Input {
    let input = fs::read_to_string(format!("data/{}/{}", DAY, file)).expect("Failed to read input file");
    println!("==== {} ====", file);
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_one(input: Input) {
    // println!("{:?}", input);
    let start = input[0].iter().position(|c| *c == 'S').unwrap();
    let mut beams = vec![0; input[0].len()];
    let mut splits = 0;
    beams[start] = 1;
    for i in 1..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == '^' {
                if beams[j] == 1 {
                    beams[j] = 0;
                    beams[j+1] = 1;
                    beams[j-1] = 1;
                    splits += 1;
                }
            }
        }

    }

    println!("Part one: {}", splits);
}

fn part_two(input: Input) {
    // println!("{:?}", input);
    let start = input[0].iter().position(|c| *c == 'S').unwrap();
    let mut beams = vec![0; input[0].len()];
    let mut splits = 0;
    beams[start] = 1;
    for i in 1..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == '^' {
                if beams[j] >= 1 {
                    
                    beams[j+1] += beams[j];
                    beams[j-1] += beams[j];
                    beams[j] = 0;
                    splits += 1;
                }
            }
        }

    }

    println!("Part two: {}", beams.iter().sum::<usize>());
}
fn main() {
    let input = read_input("example");
    part_one(input.clone());

    let input = read_input("input");
    part_one(input.clone());

    let input = read_input("example");
    part_two(input.clone());

    let input = read_input("input");
    part_two(input.clone());
}