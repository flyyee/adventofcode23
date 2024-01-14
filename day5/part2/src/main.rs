use itertools::Itertools;
use std::fs;

struct Interval {
    start: usize,
    length: usize,
}

impl Interval {
    pub fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }

    pub fn split(&mut self, offset: usize) -> Result<Self, &str> {
        if offset < self.length {
            return Err("invalid length");
        }
        let i = Self {
            start: self.start + offset,
            length: self.length - offset,
        };
        self.length = offset;
        Ok(i)
    }

    pub fn contains(&self, value: usize) -> Option<usize> {
        if self.start <= value && value < self.start + self.length {
            Some(value - self.start)
        } else {
            None
        }
    }
}

struct intersect_result_part {
    intersection: Option<Interval>,
    remainders: (Option<Interval>, Option<Interval>),
}

impl intersect_result_part {
    pub fn new() -> Self {
        Self {
            intersection: None,
            remainders: (None, None),
        }
    }
}

struct intersect_result {
    dest: intersect_result_part,
    src: intersect_result_part,
}

pub fn intersect(me: Interval, other: Interval) -> (intersect_result) {
    intersect_result {
        dest: intersect_result_part::new(),
        src: intersect_result_part::new(),
    }
}

fn main() {
    let input = fs::read_to_string("/home/kali/projects/aoc/rust23/day5/part1/src/testcase.txt")
        .expect("read fail");

    type Seed = isize;
    type Other = isize;
    let mut mapping: Vec<(Interval, Interval)> = input
        .split_once('\n')
        .expect("parse fail 1")
        .0
        .split_whitespace()
        .skip(1)
        .chunks(2)
        .into_iter()
        .map(|mut x| {
            let i = Interval::new(
                x.nth(0).unwrap().parse::<usize>().expect("parse fail 2"),
                x.nth(0).unwrap().parse::<usize>().expect("parse fail 2"),
            );
            (i, i)
        })
        .collect();

    mapping.sort_unstable_by(|x, y| Ord::cmp(&x.1, &y.1));

    for chunk in input.split("\n\n").skip(1) {
        type From = isize;
        type To = isize;
        type Count = isize;
        let mut intervals: Vec<(To, From, Count)> = chunk
            .split('\n')
            .skip(1)
            .map(|x| {
                x.split_whitespace()
                    .map(|x| x.parse::<isize>().expect("parse 3"))
                    .collect_tuple()
                    .expect("parse 4")
            })
            .collect();
        intervals.sort_unstable_by(|a, b| Ord::cmp(&a.1, &b.1));
        // println!("{intervals:?}");
        // println!("{mapping:?}");

        let mut mov = intervals.iter().peekable();
        let mut curr = intervals.first().expect("first");
        for (_, other) in mapping.iter_mut() {
            while let Some(mov_next) = mov.peek() {
                if *other >= mov_next.1 {
                    curr = mov.next().expect("next");
                } else {
                    break;
                }
            }

            let diff = *other - curr.1;
            // println!("{} {}", *other, diff);
            if 0 <= diff && diff < curr.2 {
                *other = curr.0 + diff;
            }
        }

        mapping.sort_unstable_by(|x, y| Ord::cmp(&x.1, &y.1));
    }

    // println!("{mapping:?}");
    let ans = mapping.first().expect("first").1;
    println!("{ans}");
}
