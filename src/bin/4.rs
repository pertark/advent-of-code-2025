#![allow(unused)]
use std::fs;

static DAY: u8 = 4;
type Input = Vec<Vec<char>>;

fn read_input(file: &str) -> Input {
    let input = fs::read_to_string(format!("data/{}/{}", DAY, file)).expect("Failed to read input file");
    println!("==== {} ====", file);
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_oobs(input: &Input, x: isize, y: isize) -> bool {
    let height = input.len();
    let width = input[0].len();
    x < 0 || y < 0 || x >= width as isize || y >= height as isize
}

fn accessible_by_forklift(input: &Input, ux: usize, uy: usize) -> bool {
    let x = ux as isize;
    let y = uy as isize;
    let mut sum = 0;
    if is_oobs(input, x-1, y-1) || input[ux-1][uy-1] == '.' {
        sum += 1;
    }
    if is_oobs(input, x-1, y) || input[ux-1][uy] == '.' {
        sum += 1;
    }
    if is_oobs(input, x-1, y+1) || input[ux-1][uy+1] == '.' {
        sum += 1;
    }
    if is_oobs(input, x, y-1) || input[ux][uy-1] == '.' {
        sum += 1;
    }
    if is_oobs(input, x, y+1) || input[ux][uy+1] == '.' {
        sum += 1;
    }
    if is_oobs(input, x+1, y-1) || input[ux+1][uy-1] == '.' {
        sum += 1;
    }
    if is_oobs(input, x+1, y) || input[ux+1][uy] == '.' {
        sum += 1;
    }
    if is_oobs(input, x+1, y+1) || input[ux+1][uy+1] == '.' {
        sum += 1;
    }
    sum > 4
}

fn part_one(input: Input) {
    let mut sum = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == '@' {
                if accessible_by_forklift(&input, i, j) {
                    sum += 1;
                    print!("X");
                } else {
                    print!("{}", input[i][j]);
                }
            } else {
                print!("{}", input[i][j]);
            }
        }
        println!();
    }
    println!("Part one: {}", sum);
}

fn part_two(mut input: Input) {
    let mut sum = 0;
    let mut i = 0;
    loop {
        println!("Iteration {}", i);
        let mut removed = 0;
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                if input[i][j] == '@' {
                    if accessible_by_forklift(&input, i, j) {
                        sum += 1;
                        removed += 1;
                        print!("X");
                        input[i][j] = '.';
                    } else {
                        print!("{}", input[i][j]);
                    }
                } else {
                    print!("{}", input[i][j]);
                }
            }
            println!();
        }
        
        if removed == 0 {
            break;
        }
        i += 1;
    } 
    println!("Part one: {}", sum);
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