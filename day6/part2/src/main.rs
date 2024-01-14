use std::fs;
use std::iter;

fn quadratic_solver(a: f64, b: f64, c: f64) -> (f64, f64) {
    let lhs = -b / (2.0 * a);
    let rhs = f64::sqrt(f64::powi(b, 2) - 4.0 * a * c) / (2.0 * a);
    (lhs - rhs, lhs + rhs)
}

fn main() {
    let input =
        fs::read_to_string("/home/kali/projects/aoc/rust23/day6/part1/src/testcase.txt").unwrap();
    println!("Hello, world!");
    let time = input
        .split('\n')
        .nth(0)
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("");

    let distance = input
        .split('\n')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("");

    let time = time.parse::<f64>().unwrap();
    let record_distance = distance.parse::<f64>().unwrap();

    let mut roots = quadratic_solver(-1.0, time, -record_distance);
    if roots.0 > roots.1 {
        (roots.0, roots.1) = (roots.1, roots.0);
    }
    
    let ans = f64::abs(roots.1.floor() - roots.0.ceil()) + 1.0;
    let ans = ans as usize;

    println!("{ans}");
}

fn brute_force() {
    let input =
        fs::read_to_string("/home/kali/projects/aoc/rust23/day6/part1/src/testcase.txt").unwrap();
    println!("Hello, world!");
    let time = input
        .split('\n')
        .nth(0)
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("");

    let distance = input
        .split('\n')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("");

    let mut ans = 0usize;

    let time = time.parse::<usize>().unwrap();
    let record_distance = distance.parse::<usize>().unwrap();
    for hold_duration in 0..=time {
        let speed = hold_duration;
        let time_left = time - hold_duration;
        let my_distance = speed * (time_left);
        if my_distance > record_distance {
            ans += 1;
        }
    }

    println!("{ans}");
}
