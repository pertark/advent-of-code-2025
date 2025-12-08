#![allow(unused)]
use std::fs;

static DAY: u8 = 8;
type Input = Vec<Vec<i128>>;

fn read_input(file: &str) -> Input {
    let input = fs::read_to_string(format!("data/{}/{}", DAY, file)).expect("Failed to read input file");
    println!("==== {} ====", file);
    input.lines().map(|line| {
        line.split(',').filter_map(|s| s.parse().ok()).collect()
    }).collect()
}

fn ds_find(ds: &mut Vec<isize>, x: usize) -> usize {
    if ds[x] < 0 {
        x
    } else {
        let r = ds_find(ds, ds[x] as usize);
        ds[x] = r as isize;
        r
    }
}

fn ds_union(ds: &mut Vec<isize>, x: usize, y: usize) -> bool {
    let mut rx = ds_find(ds, x);
    let mut ry = ds_find(ds, y);
    // println!("union({}, {}): {} (size {}) and {} (size {})",
    //      x, y, rx, ds[rx], ry, ds[ry]);
    if rx == ry { return false; }

    if ds[rx] > ds[ry] {
        std::mem::swap(&mut rx, &mut ry);
    }

    ds[rx] += ds[ry];
    ds[ry] = rx as isize;
    true
}

fn part_one(input: Input) {
    println!("{:?}", input);
    let n = input.len();
    let mut dist = vec![vec![0i128; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i != j {
                dist[i][j] = input[i].iter().zip(input[j].iter())
                    .map(|(a, b)| (a - b)*(a - b)).sum();
            }
        }
    }

    let mut edges = dist.iter().enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &d)| {
                if i < j {
                    Some((d, i, j))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    edges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut ds = vec![-1isize; n];
    let mut i = 0;
    for (d, u, v) in edges {
        // reading comprehension :(
        ds_union(&mut ds, u, v);
        i += 1;
        // if i == 10 {
        if i == 1000 {
            break;
        }
    }

    let mut circuits = vec![];
    for i in 0..n {
        if ds[i] < 0 {
            circuits.push(ds[i]);
        }
    }

    circuits.sort();
    println!("ds: {:?}", ds);
    println!("circuits: {:?}", circuits);

    println!("Part one: {:?}", circuits.iter().take(3).product::<isize>());

}

fn part_two(input: Input) {
    let n = input.len();
    let mut dist = vec![vec![0i128; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i != j {
                dist[i][j] = input[i].iter().zip(input[j].iter())
                    .map(|(a, b)| (a - b)*(a - b)).sum();
            }
        }
    }

    let mut edges = dist.iter().enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &d)| {
                if i < j {
                    Some((d, i, j))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    edges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut ds = vec![-1isize; n];
    let mut circuits = n;
    for (d, u, v) in edges {
        if ds_union(&mut ds, u, v) {
            circuits -= 1;
        }
        if circuits == 1 {
            println!("Part two: {:?}", input[u][0] * input[v][0]);
            break;
        }
    }
}

fn main() {
    // let input = read_input("example");
    // part_one(input.clone());

    let input = read_input("input");
    part_one(input.clone());

    let input = read_input("example");
    part_two(input.clone());

    let input = read_input("input");
    part_two(input.clone());
}