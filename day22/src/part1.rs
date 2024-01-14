use itertools::Itertools;
use std::cmp;
use std::{collections::HashMap, collections::HashSet, fs, hash::Hash};

type BrickPart = (isize, isize, isize);
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Brick(BrickPart, BrickPart);

struct BrickManager {
    brick_count: usize,
    tower: HashMap<(isize, isize), (isize, usize)>, // (x, y) => (highest z seen, brick_idx)
    crucial_bricks: HashSet<usize>,
}

impl BrickManager {
    pub fn new() -> Self {
        Self {
            brick_count: 0,
            tower: HashMap::new(),
            crucial_bricks: HashSet::new(),
        }
    }

    pub fn add_brick(&mut self, brick: &Brick) {
        self.brick_count += 1;
        let mut highest_z: Option<isize> = None;
        let mut supporting_bricks = HashSet::<usize>::new();

        for x in brick.0 .0..=brick.1 .0 {
            for y in brick.0 .1..=brick.1 .1 {
                if let Some((z, idx)) = self.tower.get(&(x, y)) {
                    if *z < highest_z.unwrap_or(0) {
                        continue;
                    } else if *z > highest_z.unwrap_or(0) {
                        supporting_bricks.clear();
                    }
                    highest_z = Some(*z);
                    supporting_bricks.insert(*idx);
                }
            }
        }

        if supporting_bricks.len() == 1 {
            self.crucial_bricks
                .insert(supporting_bricks.into_iter().nth(0).unwrap());
        }

        let my_z = highest_z.unwrap_or(0) + 1 + brick.1 .2 - brick.0 .2;
        for x in brick.0 .0..=brick.1 .0 {
            for y in brick.0 .1..=brick.1 .1 {
                self.tower
                    .entry((x, y))
                    .and_modify(|(z, idx)| {
                        if my_z > *z {
                            *z = my_z;
                            *idx = self.brick_count;
                        }
                    })
                    .or_insert((my_z, self.brick_count));
            }
        }
    }

    pub fn get_answer(&self) -> usize {
        self.brick_count - self.crucial_bricks.len()
    }
}

fn main() {
    let input =
        fs::read_to_string("/home/kali/projects/aoc/rust23/day22/src/testcase.txt").unwrap();

    let mut bricks = input
        .split('\n')
        .map(|brick| {
            let (part1, part2) = brick.split_once('~').unwrap();
            Brick(
                part1
                    .split(',')
                    .map(|c| c.parse::<isize>().unwrap())
                    .collect_tuple::<BrickPart>()
                    .unwrap(),
                part2
                    .split(',')
                    .map(|c| c.parse::<isize>().unwrap())
                    .collect_tuple::<BrickPart>()
                    .unwrap(),
            )
        })
        .collect::<Vec<Brick>>();

    bricks.sort_by(|a, b| a.0 .2.cmp(&b.0 .2));
    let mut bm = BrickManager::new();
    for brick in bricks.into_iter() {
        bm.add_brick(&brick);
    }

    println!("Ans: {}", bm.get_answer());
}
