use grid::*;
use itertools::Itertools;
use std::fs;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    #[default]
    None,
    Circle,
}

enum CycleTilt {
    North,
    West,
    South,
    East,
}

fn cycle_check<T: PartialEq>(history: &Vec<T>) -> Option<(usize, usize)> {
    let last = history.last()?;
    for (i, el) in history.iter().enumerate().rev().skip(1) {
        if el == last {
            let cycle_length = history.len() - 1 - i;
            if i - cycle_length >= 0 && history[i - cycle_length] == history[i] {
                return Some((i - cycle_length, cycle_length));
            }
        }
    }

    None
}

fn count_load(map: &Grid<Tile>) -> usize {
    let mut ans = 0usize;
    let (height, _) = map.size();
    for col in map.iter_cols() {
        for (row_number, el) in col.enumerate() {
            if matches!(el, Tile::Circle) {
                ans += height - row_number;
            }
        }
    }
    ans
}

fn main() {
    let input = fs::read_to_string("/home/kali/projects/aoc/rust23/day14/part2/src/testcase.txt")
        .unwrap()
        .trim()
        .to_owned();

    let width = input.find('\n').unwrap();
    let height = (input.len() + 1) / (width + 1);
    let mut ans = 0;

    let mut map: Grid<Tile> = Grid::new(height, width);
    let mut row_squares = vec![vec![-1isize]; height];
    let mut col_squares = vec![vec![-1isize]; width];

    for (i, row) in input.split('\n').enumerate() {
        for (j, val) in row.as_bytes().iter().enumerate() {
            if *val == b'#' {
                row_squares[i].push(j as isize);
                col_squares[j].push(i as isize);
            } else if *val == b'O' {
                unsafe {
                    *map.get_unchecked_mut(i, j) = Tile::Circle;
                }
            }
        }
    }

    for rs in row_squares.iter_mut() {
        rs.push(width as isize);
    }
    for cs in col_squares.iter_mut() {
        cs.push(height as isize);
    }

    let cycle_tilts = vec![
        CycleTilt::North,
        CycleTilt::West,
        CycleTilt::South,
        CycleTilt::East,
    ];

    let mut history: Vec<Grid<Tile>> = Vec::<Grid<Tile>>::new();

    const NUM: usize = 1000000000;
    let mut cyclical: Option<(usize, usize)> = None;
    for (i, tilt) in cycle_tilts.iter().cycle().enumerate() {
        if i % 100 == 0 {
            // TODO: Handle cycles
            if let Some(c) = cycle_check(&history) {
                cyclical = Some(c);
                break;
            }
        }

        if i > NUM * 4 {
            break;
        }

        let squares = match *tilt {
            CycleTilt::North | CycleTilt::South => &col_squares,
            CycleTilt::West | CycleTilt::East => &row_squares,
        };

        let index: fn(usize, usize) -> (usize, usize) = match *tilt {
            CycleTilt::North | CycleTilt::South => |x, y| (x, y),
            CycleTilt::West | CycleTilt::East => |x, y| (y, x),
        };

        for (lane_number, lane) in squares.iter().enumerate() {
            for (square1, square2) in lane.iter().tuple_windows() {
                if *square2 == 0 {
                    continue;
                }

                let mut circle_count = 0usize;
                let start = (*square1 + 1) as usize;
                let end = (*square2 - 1) as usize;

                let mut i = start;

                if start <= end {
                    loop {
                        unsafe {
                            let idx = index(i, lane_number);
                            if *map.get_unchecked(idx.0, idx.1) == Tile::Circle {
                                circle_count += 1;
                                *map.get_unchecked_mut(idx.0, idx.1) = Tile::None;
                            }
                        }
                        if i == end {
                            break;
                        }
                        i += 1;
                    }
                }

                i = match *tilt {
                    CycleTilt::North | CycleTilt::West => start,
                    CycleTilt::South | CycleTilt::East => end,
                };
                let offset: isize = match *tilt {
                    CycleTilt::North | CycleTilt::West => 1,
                    CycleTilt::South | CycleTilt::East => -1,
                };

                for _ in 0..circle_count {
                    unsafe {
                        let idx = index(i, lane_number);
                        *map.get_unchecked_mut(idx.0, idx.1) = Tile::Circle;
                    }
                    i = (i as isize + offset) as usize;
                }
            }
        }

        if i % 4 == 3 {
            history.push(map.clone());
        }
    }

    if let Some((start, length)) = cyclical {
        println!("cycle of {length} from {start}");
        ans = count_load(&history[(NUM - 1 - start) % length + start]);
    } else {
        ans = count_load(history.last().unwrap());
    }

    println!("{ans}");
}
