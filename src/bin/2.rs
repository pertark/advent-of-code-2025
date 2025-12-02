use std::fs;

static DAY: u8 = 2;
type Input = Vec<[String; 2]>;

fn read_input(file: &str) -> Input {
    let input = fs::read_to_string(format!("data/{}/{}", DAY, file)).expect("Failed to read input file");
    println!("==== {} ====", file);
    input.split(",").map(|line| {
        let mut parts = line.split("-");
        [
            parts.next().unwrap_or_default().to_string(),
            parts.next().unwrap_or_default().to_string(),
        ]
    }).collect()
}

fn check_id_invalid(id: String) -> bool {
    // easy method is just to divide string in half and then 
    // check if they're equal
    let len = id.len();
    if (len % 2) != 0 {
        return false;
    }

    let (first_half, second_half) = id.split_at(len / 2);
    first_half == second_half
}

fn part_one(input: Input) {
    println!("{:?}", input);
    let mut sum: u128 = 0;
    println!("{} pairs", input.len());
    for pair in input {
        let a = pair[0].parse().unwrap_or(0u128);
        let b = pair[1].parse().unwrap_or(0u128);
        for id in a..=b {
            if check_id_invalid(id.to_string()) {
                sum += id;
                // println!("{} ", id);
            }
        }
        println!("Processed pair: {:?}, current sum: {}", pair, sum);
    }

    println!("Sum of invalid IDs: {}", sum);
}

fn check_id_invalid_two(id: String) -> bool {
    let len = id.len();
    let mut factors = Vec::new();
    for i in 2..=len {
        if len % i == 0 {
            factors.push(i);
        }
    }
    // println!("{:?}", factors);
    if factors.len() == 0 {
        return false;
    }

    // highly unidiomatic :(
    for factor in factors {
        let chunk_size = len / factor;
        let mut all_equal = true;
        for i in 0..factor-1 {
            let start1 = i * chunk_size;
            let end1 = start1 + chunk_size;
            let start2 = (i + 1) * chunk_size;
            let end2 = start2 + chunk_size;
            if &id[start1..end1] != &id[start2..end2] {
                all_equal = false;
                break;
            }
        }
        if all_equal {
            return true;
        }
    }

    false
}

fn part_two(input: Input) {
    println!("{:?}", input);
    let mut sum: u128 = 0;
    println!("{} pairs", input.len());
    for pair in input {
        let a = pair[0].parse().unwrap_or(0u128);
        let b = pair[1].parse().unwrap_or(0u128);
        for id in a..=b {
            if check_id_invalid_two(id.to_string()) {
                sum += id;
                // println!("{} ", id);
            }
        }
        println!("Processed pair: {:?}, current sum: {}", pair, sum);
    }

    println!("Sum of invalid IDs: {}", sum);
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