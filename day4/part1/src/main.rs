use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("/home/kali/projects/aoc/rust23/day4/part1/src/testcase.txt")
        .expect("input file read fail");

    let ans: usize = input
        .split('\n')
        .map(|line| {
            let (_, line) = line.split_once(": ").expect("parse line fail");
            let (winners, mines) = line.split_once(" | ").expect("parse line fail 2");
            let mut winners = winners
                .split_whitespace()
                .map(|winner| {
                    let key = winner.parse::<usize>().expect("Fail to parse usize");
                    (key, 0)
                })
                .collect::<HashMap<usize, usize>>();

            println!("{winners:?}");
            mines
                .split_whitespace()
                .map(|mine| mine.parse::<usize>().expect("Fail to parse usize"))
                .for_each(|mine| {
                    winners.entry(mine).and_modify(|count| *count += 1);
                });
            println!("{winners:?}");

            let i = winners.into_iter().filter(|(_x, y)| y != &0).count();
            // .map(|(_x, y)| (1 << (y - 1)) - 1)
            // .sum::<usize>();
            if i == 0 {
                0
            } else {
                1 << (i - 1)
            }
        })
        .sum();

    println!("{ans}");
}
