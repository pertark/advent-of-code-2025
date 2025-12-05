#![allow(unused)]
use std::fs;

static DAY: u8 = 5;
type Input = (Vec<[u128; 2]>, Vec<u128>);

fn read_input(file: &str) -> Input {
    let input = fs::read_to_string(format!("data/{}/{}", DAY, file)).expect("Failed to read input file");
    println!("==== {} ====", file);
    let raw: Vec<String> = input.split("\r\n\r\n").map(|s| s.to_string()).collect();
    let ids = raw[0].lines().map(|s| {
        let parts: Vec<u128> = s.split("-").map(|p| p.trim().parse().unwrap()).collect();
        [parts[0], parts[1]]
    }).collect();
    let ingredients = raw[1].lines().map(|s| s.parse().unwrap()).collect();
    (ids, ingredients)
}

fn part_one(input: Input) {
    let (ids, ingredients) = input;
    let mut fresh = 0;
    for ingredient in ingredients {
        for ids in &ids {
            if ingredient >= ids[0] && ingredient <= ids[1] {
                fresh += 1;
                break;
            }
        }
    }
    println!("Part one: {}", fresh);
}

fn part_two(input: Input) {
    let (mut ids, _) = input;
    ids.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut fresh = 0;
    let mut current_start = 1;
    let mut current_end = 0;
    for id in ids {
        if id[0] > current_end + 1 {
            // println!("adding! {}-{}", current_start, current_end);
            fresh += current_end + 1 - current_start; // unsigned shenanigans 😪
            current_start = id[0];
            current_end = id[1];
        } else if id[1] > current_end {
            current_end = id[1];
        }
        // println!("current: {}-{}", current_start, current_end);
    }
    fresh += current_end + 1 - current_start;
    println!("Part two: {}", fresh);
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