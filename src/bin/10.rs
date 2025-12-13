#![allow(unused)]
use std::{collections::HashMap, fs, collections::VecDeque};

static DAY: u16 = 10;
type Input = Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<u16>)>;

fn parse_line(line: &str) -> (Vec<bool>, Vec<Vec<usize>>, Vec<u16>) {
    let lights_buttons = line.split("{").nth(0).unwrap().trim();
    let joltage = line.split("{").nth(1).unwrap().trim().trim_end_matches("}").trim()
        .split(",").map(|s| s.trim().parse().unwrap()).collect();
    let lights = lights_buttons.split(" ").nth(0).unwrap().trim()
        .trim_matches(|c| c == '[' || c == ']').chars()
        .map(|c| c == '#').collect();
    let mut buttons: Vec<Vec<usize>> = lights_buttons.split(" ").into_iter().skip(1)
        .map(|s| s.trim_matches(|c| c == '(' || c == ')').split(',')
        .map(|s| s.trim().parse::<usize>().unwrap()).collect()).collect();
    buttons.sort_by_key(|b| b.len());
    buttons.reverse();
    println!("Number of buttons: {}", buttons.len());
    (lights, buttons, joltage)
}

fn read_input(file: &str) -> Input {
    let input = fs::read_to_string(format!("data/{}/{}", DAY, file)).expect("Failed to read input file");
    println!("==== {} ====", file);
    input.lines().map(|line| parse_line(line)).collect()
}

fn part_one_bash(state: Vec<bool>, objective: &Vec<bool>, buttons: &Vec<Vec<usize>>) -> i128 {
    let mut queue = VecDeque::new();
    queue.push_back((state, 0));
    while let Some((current, steps)) = queue.pop_front() {
        if current == *objective {
            return steps;
        }
        for button in buttons {
            let mut new_state = current.clone();
            for &idx in button {
                new_state[idx as usize] = !new_state[idx as usize];
            }
            queue.push_back((new_state, steps + 1));
        }
    }
    i128::MAX
}

fn part_one(input: Input) {
    let mut total = 0;
    for (light, buttons, _) in &input {
        println!("Target: {light:?}");
        let mut state = vec![false; light.len()];
        let result = part_one_bash(state, light, buttons);
        println!("Result for light {:?} is {}", light, result);
        total += result;
    }
    println!("Part one: {total}");
}

fn format_state(state: &Vec<bool>) -> String {
    let s: String = state.iter().map(|&b| if b {'#'} else {'.'}).collect();
    format!("[{}]", s)
}

struct S {
    dp: HashMap<Vec<u16>, i128>,
}

impl S {
    fn new() -> Self {
        S {
            dp: HashMap::new(),
        }
    }
    pub fn hash_joltage() {

    }
    pub fn bash(&mut self, joltage: Vec<u16>,
        buttons: &Vec<Vec<usize>>, joltage_requirements: &Vec<u16>) -> i128 {
        match self.dp.get(&(joltage.clone())) {
            Some(&res) => {
                res
            },
            None => {
                let result = self._bash(joltage.clone(),
                    buttons, joltage_requirements);
                self.dp.insert(joltage.clone(), result);
                result
            }
        }
    }
    pub fn _bash(&mut self, joltage: Vec<u16>,
        buttons: &Vec<Vec<usize>>, joltage_requirements: &Vec<u16>) -> i128 {
        if joltage == *joltage_requirements {
            println!("Found solution!");
            return 0;
        }
        if joltage.iter().zip(joltage_requirements.iter()).any(|(a, b)| a > b) {
            return i128::MAX;
        }
        let mut best = i128::MAX;
        for lights in buttons {

            let mut new_joltage = joltage.clone();
            for &light in lights {
                new_joltage[light] += 1;
            }
            let res = self.bash(new_joltage, buttons, joltage_requirements);
            if res != i128::MAX {
                best = best.min(res + 1);
            }
        }
        // println!("Best for joltage {:?}, {}", joltage, best);
        best
    }
}

// fail. check python solution
fn part_two(input: Input) {
    let mut total = 0;
    for (_, buttons, joltage_requirements) in &input {
        println!("Buttons: {:?}", buttons);
        println!("Joltage req: {:?}", joltage_requirements);
        let mut joltage = vec![0u16; joltage_requirements.len()];
        let mut s = S::new();
        let result = s.bash(joltage, buttons, joltage_requirements);
        println!("Debug DP size: {}", s.dp.len());
        println!("Result {result}");
        // unreachable!();
        println!("Result for joltage {:?} is {}", joltage_requirements, result);
        total += result;
    }
    println!("Part two: {total}");
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