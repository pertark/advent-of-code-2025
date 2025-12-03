#![allow(unused)]
use std::fs;

static DAY: u8 = 3;
type Input = Vec<String>;

fn read_input(file: &str) -> Input {
    let input = fs::read_to_string(format!("data/{}/{}", DAY, file)).expect("Failed to read input file");
    println!("==== {} ====", file);
    input.lines().map(|line| line.to_string()).collect()
}

fn part_one(input: Input) {
    // println!("{:?}", input);
    let mut sum = 0;
    for bank in input {
        // mono stack
        let mut stack = Vec::new();

        for char in bank.chars().collect::<Vec<_>>()[..bank.len()-1].iter() {
            let digit = char.to_digit(10).unwrap();
            while let Some(&top) = stack.last() {
                if top < digit {
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(digit);
        }

        let last = bank.chars().collect::<Vec<_>>()[bank.len()-1].to_digit(10).unwrap();
        let joltage = if stack.len() >= 2 {
            std::cmp::max(10 * stack[0] + stack[1], 10 * stack[0] + last)
        } else {
            10 * stack[0] + last
        };

        sum += joltage; 
    }
    println!("Part one: {}", sum);
}

fn part_two(input: Input) {
    // idk why i did it the other way when this way works for both
    let mut sum = 0;
    for bank in input {
        let mut stack = Vec::new();

        for char in bank.chars() {
            let digit = char.to_digit(10).unwrap();
            if stack.len() < 12 {
                stack.push(digit);
            } else {
                let mut cmin = stack[0];
                for i in 1..stack.len() {
                    if stack[i] > cmin {
                        stack.remove(i-1);
                        stack.push(digit);
                        break;
                    }
                    cmin = stack[i];
                }
                if stack[stack.len()-1] < digit {
                    stack.remove(stack.len()-1);
                    stack.push(digit);
                }
            }
        }

        let mut joltage: u128 = 0;
        for digit in stack {
            joltage = joltage * 10 + digit as u128;
        }

        println!("Bank: {}, Joltage: {}", bank, joltage);
        sum += joltage; 
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