use std::fs;
use std::mem;

enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

fn find_reflection(maze: &str) -> Option<Reflection> {
    let grid = maze.split('\n').collect::<Vec<&str>>();
    let mut candidates = (1..grid[0].len()).collect::<Vec<usize>>();
    for row in 0..grid.len() {
        if candidates.len() == 0 {
            break;
        }
        let mut new_candidates = Vec::<usize>::new();
        for candidate in candidates.iter() {
            let mut i = 0usize;
            let mut symmetrical = true;
            while *candidate >= i + 1 && candidate + i < grid[0].len() {
                if grid[row].as_bytes()[candidate - 1 - i] != grid[row].as_bytes()[candidate + i] {
                    symmetrical = false;
                    break;
                }
                i += 1;
            }
            if symmetrical {
                new_candidates.push(*candidate);
            }
        }

        mem::swap(&mut candidates, &mut new_candidates);
    }

    if candidates.len() == 1 {
        return Some(Reflection::Vertical(candidates[0]));
    }

    // WET CODE
    let mut candidates = (1..grid.len()).collect::<Vec<usize>>();
    for col in 0..grid[0].len() {
        if candidates.len() == 0 {
            break;
        }
        let mut new_candidates = Vec::<usize>::new();
        for candidate in candidates.iter() {
            let mut i = 0usize;
            let mut symmetrical = true;
            while *candidate >= 1 + i && candidate + i < grid.len() {
                if grid[candidate - 1 - i].as_bytes()[col] != grid[candidate + i].as_bytes()[col] {
                    symmetrical = false;
                    break;
                }
                i += 1;
            }
            if symmetrical {
                new_candidates.push(*candidate);
            }
        }

        mem::swap(&mut candidates, &mut new_candidates);
    }

    if candidates.len() == 1 {
        return Some(Reflection::Horizontal(candidates[0]));
    }

    None
}

fn main() {
    let input =
        fs::read_to_string("/home/kali/projects/aoc/rust23/day13/part1/src/testcase.txt").unwrap();
    let ans: usize = input
        .split("\n\n")
        .map(
            |maze| match find_reflection(maze).expect("Failed to find reflection") {
                Reflection::Horizontal(x) => 100 * x,
                Reflection::Vertical(x) => x,
            },
        )
        .sum();

    println!("{ans}");
}
