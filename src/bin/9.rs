#![allow(unused)]
use std::fs;

static DAY: u8 = 9;
type Input = Vec<Vec<i128>>;

fn read_input(file: &str) -> Input {
    let input = fs::read_to_string(format!("data/{}/{}", DAY, file)).expect("Failed to read input file");
    println!("==== {} ====", file);
    input.lines().map(|line| {
        line.split(',').filter_map(|s| s.parse().ok()).collect()
    }).collect()
}

fn part_one(input: Input) {
    // println!("{:?}", input);
    let n = input.len();
    let mut max_area = 0;
    for i in 0..n {
        for j in i+1..n {
            let area = (input[i][0] - input[j][0] + 1).abs() * (1 + input[i][1] - input[j][1]).abs();
            max_area = max_area.max(area);
        }
    }
    println!("Part one: {}", max_area);
}

fn detect_intersect(edges: &Vec<(i128, i128, i128)>, b: i128, a1: i128, a2: i128) -> bool {
    // assert!(a1 < a2);
    if a1 > a2 {
        return detect_intersect(edges, b, a2, a1);
    }

    for &(x, y1, y2) in edges {
        // we do not care if the edge intersects at the corner
        if x > a1 && x < a2 { 
            // but we do care if it starts at the middle of our box edge
            if (b >= y1 && b <= y2) || (b <= y1 && b >= y2) { 
                return true;
            }
        }
    }
    false
}

fn part_two(input: Input) {
    let n = input.len();
    let mut horizontal_edges = vec![];
    let mut vertical_edges = vec![];
    for i in 0..n-1 {
        let x1 = input[i][0];
        let y1 = input[i][1];
        let x2 = input[i+1][0];
        let y2 = input[i+1][1];
        if x1 == x2 {
            vertical_edges.push((x1, y1, y2));
        } else {
            horizontal_edges.push((y1, x1, x2));
        }
    }
    {
        let x1 = input[n-1][0];
        let y1 = input[n-1][1];
        let x2 = input[0][0];
        let y2 = input[0][1];
        if x1 == x2 {
            vertical_edges.push((x1, y1, y2));
        } else {
            horizontal_edges.push((y1, x1, x2));
        }
    }

    let mut max_area = 0;
    for i in 0..n {
        for j in i+1..n {
            let (x1, y1) = (input[i][0], input[i][1]);
            let (x2, y2) = (input[j][0], input[j][1]);
            if detect_intersect(&vertical_edges, y1, x1, x2) 
            || detect_intersect(&horizontal_edges, x1, y1, y2)
            || detect_intersect(&vertical_edges, y2, x1, x2)
            || detect_intersect(&horizontal_edges, x2, y1, y2) {
                continue;
            }

            let area = ((input[i][0] - input[j][0]).abs() + 1) * (1 + (input[i][1] - input[j][1]).abs());
            // println!("Found area {} between points {:?} and {:?}", area, input[i], input[j]);
            max_area = max_area.max(area);
        }
    }
    println!("Part two: {}", max_area);
}

fn main() {
    let input = read_input("example");
    part_one(input.clone());

    let input = read_input("input");
    part_one(input.clone());

    let input = read_input("example");
    part_two(input.clone());

    let input = read_input("example copy");
    part_two(input.clone());

    let input = read_input("input");
    part_two(input.clone());
}