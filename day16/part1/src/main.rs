use enum_map::{Enum, EnumMap};
use std::{collections::HashMap, fs};

#[derive(Debug, Enum, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
type Transformer = fn(&Direction) -> Vec<Direction>;
struct Tile<'a> {
    transformer: &'a Transformer,
    seen: EnumMap<Direction, bool>,
}

impl<'a> Tile<'a> {
    pub fn new(transformer: &'a Transformer) -> Self {
        Self {
            transformer,
            seen: EnumMap::default(),
        }
    }

    pub fn transform(&mut self, direction: &Direction) -> Vec<Direction> {
        if self.seen[*direction] {
            return vec![];
        }
        self.seen[*direction] = true;

        (self.transformer)(direction)
    }

    pub fn is_energized(&self) -> bool {
        self.seen.values().any(|q| *q)
    }
}

fn main() {
    let input =
        fs::read_to_string("/home/kali/projects/aoc/rust23/day16/part1/src/testcase.txt").unwrap();
    let mut transformers = HashMap::<u8, Transformer>::new();
    transformers.insert(b'.', |d| vec![*d]);
    transformers.insert(b'/', |d| {
        use Direction::*;
        vec![match *d {
            Up => Right,
            Right => Up,
            Down => Left,
            Left => Down,
        }]
    });
    transformers.insert(b'\\', |d| {
        use Direction::*;
        vec![match *d {
            Up => Left,
            Left => Up,
            Down => Right,
            Right => Down,
        }]
    });
    transformers.insert(b'-', |d| {
        use Direction::*;
        match *d {
            Up | Down => vec![Left, Right],
            _ => vec![*d],
        }
    });
    transformers.insert(b'|', |d| {
        use Direction::*;
        match *d {
            Left | Right => vec![Up, Down],
            _ => vec![*d],
        }
    });

    let mut grid = input
        .split('\n')
        .map(|s| {
            s.as_bytes()
                .iter()
                .map(|b| Tile::new(&transformers[b]))
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<_>>();

    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    type Pos = (isize, isize);
    let mut pos: Pos = (0, -1);
    let mut direction = Direction::Right;

    let mut remaining = Vec::<(Pos, Direction)>::new();

    loop {
        loop {
            match direction {
                Direction::Up => pos.0 -= 1,
                Direction::Down => pos.0 += 1,
                Direction::Left => pos.1 -= 1,
                Direction::Right => pos.1 += 1,
            };
            if pos.0 < 0 || pos.0 >= height || pos.1 < 0 || pos.1 >= width {
                break;
            }
            let new_directions = grid[pos.0 as usize][pos.1 as usize].transform(&direction);
            if new_directions.is_empty() {
                break;
            }

            direction = new_directions[0];
            if new_directions.len() == 2 {
                remaining.push((pos, new_directions[1]));
            }
        }

        if remaining.is_empty() {
            break;
        }

        (pos, direction) = remaining.pop().unwrap();
    }

    let mut energized_count = 0usize;
    for row in grid.iter() {
        for tile in row.iter() {
            if tile.is_energized() {
                energized_count += 1;
            }
        }
    }
    println!("{energized_count}");
}
