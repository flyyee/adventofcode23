use std::fs;
use std::iter;

fn main() {
    let input = fs::read_to_string("/home/kali/projects/aoc/rust23/day6/part1/src/testcase.txt").unwrap();
    println!("Hello, world!");
    let times = input
        .split('\n')
        .nth(0)
        .unwrap()
        .split_ascii_whitespace()
        .skip(1);
    let distances = input
        .split('\n')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .skip(1);

    let mut ans = 1usize;
    for (time, distance) in iter::zip(times, distances) {
        let time = time.parse::<usize>().unwrap();
        let record_distance = distance.parse::<usize>().unwrap();
        let mut possibilities = 0usize;
        for hold_duration in 0..=time {
            let speed = hold_duration;
            let time_left = time - hold_duration;
            let my_distance = speed * (time_left);
            if my_distance > record_distance {
                possibilities += 1;
            }
        }

        ans *= possibilities;
    }

    println!("{ans}");
}
