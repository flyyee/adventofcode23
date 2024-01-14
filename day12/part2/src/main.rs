use std::{collections::HashMap, fs};

mod Data {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Condition {
        Operational,
        Damaged,
        Unknown,
    }

    use std::collections::HashMap;
    use std::ops::Not;
    impl Not for Condition {
        type Output = Self;
        fn not(self) -> Self {
            match self {
                Self::Operational => Self::Damaged,
                Self::Damaged => Self::Operational,
                Self::Unknown => Self::Unknown,
            }
        }
    }

    pub fn new_conditions(s: &str) -> Result<Vec<Condition>, ()> {
        let s = [s; 5];
        let s = s.join("?");

        use Condition::*;
        let condition_list: Vec<Condition> = s
            .bytes()
            .map(|b| match b {
                b'.' => Ok(Operational),
                b'#' => Ok(Damaged),
                b'?' => Ok(Unknown),
                _ => Err(()),
            })
            .collect::<Result<Vec<Condition>, ()>>()?;

        Ok(condition_list)
    }

    pub fn is_contiguously(conditions: &[Condition], n: usize, what: Condition) -> bool {
        if n > conditions.len() {
            return false;
        }
        conditions[..n].iter().all(|x| *x != !what)
    }

    #[derive(Debug)]
    pub struct Chunk {
        variant: Condition,
        length: usize,
    }

    impl Chunk {
        fn new(variant: Condition, length: usize) -> Self {
            Self { variant, length }
        }

        pub fn length(&self) -> usize {
            self.length
        }
    }

    use std::iter;
    pub fn new_chunks(s: &str) -> Result<Vec<Chunk>, ()> {
        use Condition::*;
        let mut condition_list: Vec<Chunk> = vec![Chunk::new(Operational, 0)];

        condition_list.extend(
            iter::repeat(s.split(','))
                .take(5)
                .flatten()
                .flat_map(|b| -> Vec<Result<Chunk, ()>> {
                    if let Ok(n) = b.parse::<usize>() {
                        vec![Ok(Chunk::new(Damaged, n)), Ok(Chunk::new(Operational, 1))]
                    } else {
                        vec![Err(())]
                    }
                })
                .collect::<Result<Vec<Chunk>, ()>>()?,
        );

        *condition_list.last_mut().unwrap() = Chunk::new(Operational, 0);

        Ok(condition_list)
    }

    pub fn find_ways(
        conditions: &[Condition],
        chunks: &[Chunk],
        min_remainder: usize,
        mut memo: &mut Vec<Vec<Option<usize>>>,
    ) -> usize {
        if let Some(result) = memo[conditions.len()][chunks.len()] {
            return result;
        }

        if min_remainder > conditions.len() {
            memo[conditions.len()][chunks.len()] = Some(0);
            return 0;
        }

        if chunks.first().is_none() {
            memo[conditions.len()][chunks.len()] = Some(0);
            return 0;
        }

        let chunk = chunks.first().unwrap();
        if min_remainder < chunk.length {
            memo[conditions.len()][chunks.len()] = Some(0);
            return 0;
        }

        if chunks.len() == 1 {
            if is_contiguously(conditions, conditions.len(), chunk.variant) {
                memo[conditions.len()][chunks.len()] = Some(1);
                return 1;
            } else {
                memo[conditions.len()][chunks.len()] = Some(0);
                return 0;
            }
        }

        let mut ways = 0usize;

        if chunk.variant == Condition::Damaged {
            if is_contiguously(conditions, chunk.length, chunk.variant) {
                ways += find_ways(
                    &conditions[chunk.length..],
                    &chunks[1..],
                    min_remainder - chunk.length,
                    memo,
                );
            }
        } else {
            for chunk_size in chunk.length..=conditions.len() {
                if !is_contiguously(conditions, chunk_size, chunk.variant) {
                    break;
                }
                ways += find_ways(
                    &conditions[chunk_size..],
                    &chunks[1..],
                    min_remainder - chunk.length,
                    memo,
                );
            }
        }

        memo[conditions.len()][chunks.len()] = Some(ways);
        ways
    }
}

fn main() {
    let input =
        fs::read_to_string("/home/kali/projects/aoc/rust23/day12/part1/src/testcase.txt").unwrap();

    let mut i = 0usize;
    let ans: usize = input
        .split('\n')
        .map(|record| {
            i += 1;
            let (conditions, chunks) = record.split_once(' ').unwrap();
            let conditions: Vec<Data::Condition> = Data::new_conditions(conditions).unwrap();
            let chunks = Data::new_chunks(chunks).unwrap();
            let mut memo = vec![vec![None; chunks.len() + 1]; conditions.len() + 1];
            Data::find_ways(
                &conditions,
                &chunks,
                chunks.iter().map(|c| c.length()).sum(),
                &mut memo,
            )
        })
        .sum();
    println!("{ans}");
}
