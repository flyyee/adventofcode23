use std::cmp;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("/home/kali/projects/aoc/rust23/day4/part1/src/testcase.txt")
        .expect("input file read fail");

    let mut scores = vec![1; input.matches('\n').count() + 1];
    for (x, line) in input.split('\n').enumerate() {
        let (_, line) = line.split_once(": ").expect("parse line fail");
        let (winners, mines) = line.split_once(" | ").expect("parse line fail 2");
        let mut winners = winners
            .split_whitespace()
            .map(|winner| {
                let key = winner.parse::<usize>().expect("Fail to parse usize");
                (key, 0)
            })
            .collect::<HashMap<usize, usize>>();

        mines
            .split_whitespace()
            .map(|mine| mine.parse::<usize>().expect("Fail to parse usize"))
            .for_each(|mine| {
                winners.entry(mine).and_modify(|count| *count += 1);
            });

        let i = winners.into_iter().filter(|(_x, y)| y != &0).count();

        // populate winners
        if i != 0 {
            let curr = scores[x];
            let start = cmp::min(scores.len() - 1, x + 1);
            let end = cmp::min(scores.len(), x + i + 1);
            for score in scores[start..end].iter_mut() {
                *score += curr;
            }
        }
    }

    println!("{}", scores.iter().sum::<usize>());
}
