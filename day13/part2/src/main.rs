use std::fs;
use std::mem;

enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

fn find_reflection(maze: &str) -> Option<Reflection> {
    let grid = maze.split('\n').collect::<Vec<&str>>();
    let mut candidates = (1..grid[0].len())
        .map(|x| (x, 0))
        .collect::<Vec<(usize, usize)>>();

    for row in 0..grid.len() {
        if candidates.len() == 0 {
            break;
        }
        let mut new_candidates = Vec::<(usize, usize)>::new();

        for (candidate, smudges) in candidates.iter_mut() {
            let mut i = 0usize;
            let mut symmetrical = true;
            while *candidate >= i + 1 && *candidate + i < grid[0].len() {
                if grid[row].as_bytes()[*candidate - 1 - i] != grid[row].as_bytes()[*candidate + i]
                {
                    *smudges += 1;
                    if *smudges > 1 {
                        symmetrical = false;
                        break;
                    }
                }
                i += 1;
            }
            if symmetrical {
                new_candidates.push((*candidate, *smudges));
            }
        }

        mem::swap(&mut candidates, &mut new_candidates);
    }

    for (candidate, smudges) in candidates {
        if smudges == 1 {
            return Some(Reflection::Vertical(candidate));
        }
    }

    // WET CODE
    let mut candidates = (1..grid.len())
        .map(|x| (x, 0))
        .collect::<Vec<(usize, usize)>>();

    for col in 0..grid[0].len() {
        if candidates.len() == 0 {
            break;
        }
        let mut new_candidates = Vec::<(usize, usize)>::new();
        for (candidate, smudges) in candidates.iter_mut() {
            let mut i = 0usize;
            let mut symmetrical = true;
            while *candidate >= 1 + i && *candidate + i < grid.len() {
                if grid[*candidate - 1 - i].as_bytes()[col] != grid[*candidate + i].as_bytes()[col]
                {
                    *smudges += 1;
                    if *smudges > 1 {
                        symmetrical = false;
                        break;
                    }
                }
                i += 1;
            }
            if symmetrical {
                new_candidates.push((*candidate, *smudges));
            }
        }

        mem::swap(&mut candidates, &mut new_candidates);
    }

    for (candidate, smudges) in candidates {
        if smudges == 1 {
            return Some(Reflection::Horizontal(candidate));
        }
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
