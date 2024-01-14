use std::collections::HashMap;
use std::fs;

fn main() {
    let input =
        fs::read_to_string("/home/kali/projects/aoc/rust23/day8/part1/src/testcase.txt").unwrap();
    let instructions = input.split('\n').nth(0).unwrap();

    let directions: HashMap<String, (String, String)> = input
        .split('\n')
        .skip(2)
        .map(|x| {
            let lhs = x[0..3].to_owned();
            let rhs1 = x[7..10].to_owned();
            let rhs2 = x[12..15].to_owned();
            (lhs, (rhs1, rhs2))
        })
        .collect();

    println!("{directions:?}");

    let mut curr: &str = "AAA";
    let mut steps = 0usize;
    for instruction in instructions.as_bytes().iter().cycle() {
        match &instruction {
            b'L' => curr = &directions[curr].0[..],
            b'R' => curr = &directions[curr].1[..],
            _ => panic!("fail match"),
        }
        steps += 1;
        if curr == "ZZZ" {
            break;
        }
    }

    println!("{steps}");
}
