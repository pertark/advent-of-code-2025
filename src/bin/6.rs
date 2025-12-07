#![allow(unused)]
use std::fs;

static DAY: u8 = 6;
type Input = Vec<Vec<String>>;

fn read_input(file: &str) -> Input {
    let input = fs::read_to_string(format!("data/{}/{}", DAY, file)).expect("Failed to read input file");
    println!("==== {} ====", file);
    let lines: Vec<&str> = input.lines().collect();
    let mut chunk_sizes = vec![];
    let chars = lines[lines.len()-1].chars();
    // this is striaght up the worst parsing ive ever done
    let mut current_chunk = 0;
    for c in chars {
        if !c.is_whitespace() {
            if current_chunk > 0 {
                chunk_sizes.push(current_chunk - 1);
                current_chunk = 1;
            } else {
                current_chunk += 1;
            }
        } else {
            current_chunk += 1;
        }
    }
    if current_chunk > 0 {
        chunk_sizes.push(current_chunk);
    }
    println!("Chunk sizes: {:?}", chunk_sizes);
    input.lines().map(|line| {
        let mut parts = vec![];
        let mut i = 0;
        for size in &chunk_sizes {
            let part = &line[i..i + *size];
            parts.push(part.to_string());
            i += *size + 1;
        }
        parts
    }).collect()
}

fn part_one(input: Input) {
    let mut sum = 0;
    for i in 0..input[0].len() {
        let mut operands: Vec<u128> = vec![];
        for line in &input[..input.len()-1] {
            operands.push(line[i].parse().unwrap());
        }
        let operator = &input[input.len()-1][i];
        sum += match operator.as_str() {
            "+" => operands.iter().sum::<u128>(),
            "*" => operands.iter().product::<u128>(),
            _ => panic!("{}", operator),
        }
    }

    println!("Part one: {}", sum);
}

fn part_two(input: Input) {
    // println!("Input: {:?}", input);
    let mut sum = 0;
    for i in 0..input[0].len() {
        let chunk_size = input[0][i].len();
        let mut operands: Vec<u128> = vec![0u128; chunk_size];
        for j in 0..input.len()-1 {
            let word = &input[j][i];
            for k in 0..chunk_size {
                // println!("line[{}][{}] = {}, cs={}", j, k, line[k], chunk_size);
                operands[k] = match word.chars().nth(k).unwrap().to_string().parse::<u128>() {
                    Ok(v) => 10 * operands[k] + v,
                    Err(_) => operands[k],
                }
            }
        }
        let operator = &input[input.len()-1][i];
        println!("operands: {:?}", operands);
        sum += match operator.as_str().trim() {
            "+" => operands.iter().sum::<u128>(),
            "*" => operands.iter().product::<u128>(),
            _ => panic!("{}", operator),
        }
    }

    println!("Part two: {}", sum);
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