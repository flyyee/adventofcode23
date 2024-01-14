use pathfinding::directed::dijkstra;
use std::fs;
use std::ops::Not;
use std::slice::Iter;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Not for Direction {
    type Output = Self;
    fn not(self) -> <Self as Not>::Output {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        use Direction::*;
        static DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];
        DIRECTIONS.iter()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Node {
    pos: (usize, usize),
    last_direction: Direction,
    last_direction_count: usize,
}

impl Node {
    pub fn new(
        pos: (usize, usize),
        last_direction: Direction,
        last_direction_count: usize,
    ) -> Self {
        Self {
            pos,
            last_direction,
            last_direction_count,
        }
    }
}

fn part1(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let grid = input
        .split('\n')
        .map(|r| {
            r.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();
    let target = (grid.len() - 1, grid[0].len() - 1);

    let path = dijkstra::dijkstra(
        &Node::new((0, 0), Direction::Up, 0),
        |curr_node| {
            let mut result = Vec::<(Node, usize)>::new();
            for direction in Direction::iterator() {
                use Direction::*;
                let offset_pos: (isize, isize) = match *direction {
                    Up => (-1, 0),
                    Down => (1, 0),
                    Left => (0, -1),
                    Right => (0, 1),
                };

                if curr_node.pos.0 as isize + offset_pos.0 < 0
                    || curr_node.pos.0 as isize + offset_pos.0 >= height as isize
                {
                    continue;
                }
                if curr_node.pos.1 as isize + offset_pos.1 < 0
                    || curr_node.pos.1 as isize + offset_pos.1 >= width as isize
                {
                    continue;
                }

                let new_pos = (
                    (curr_node.pos.0 as isize + offset_pos.0) as usize,
                    (curr_node.pos.1 as isize + offset_pos.1) as usize,
                );
                let cost = grid[new_pos.0][new_pos.1];

                if curr_node.last_direction == !*direction {
                    // No reversing!
                    continue;
                } else if curr_node.last_direction == *direction {
                    if curr_node.last_direction_count == 3 {
                        continue;
                    }
                    result.push((
                        Node::new(new_pos, *direction, curr_node.last_direction_count + 1),
                        cost,
                    ));
                } else {
                    result.push((Node::new(new_pos, *direction, 1), cost));
                }
            }
            result
        },
        |curr_node| curr_node.pos == target,
    );

    if let Some((path, cost)) = path {
        // println!("{path:?}");
        println!("cost: {cost}");
    } else {
        println!("Failed to find path");
    }
}

fn part2(path: &str) {
    let input = fs::read_to_string(path).unwrap();
    let grid = input
        .split('\n')
        .map(|r| {
            r.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();
    let target = (grid.len() - 1, grid[0].len() - 1);

    let path = dijkstra::dijkstra(
        &Node::new((0, 0), Direction::Up, 0),
        |curr_node| {
            let mut result = Vec::<(Node, usize)>::new();
            for direction in Direction::iterator() {
                if curr_node.last_direction == !*direction {
                    // No reversing!
                    continue;
                } else if curr_node.last_direction_count > 0 && curr_node.last_direction_count < 4
                    && *direction != curr_node.last_direction
                {
                    // Min 4
                    continue;
                } else if curr_node.last_direction_count >= 10
                    && *direction == curr_node.last_direction
                {
                    // Max 10
                    continue;
                }

                use Direction::*;
                let offset_pos: (isize, isize) = match *direction {
                    Up => (-1, 0),
                    Down => (1, 0),
                    Left => (0, -1),
                    Right => (0, 1),
                };

                if curr_node.pos.0 as isize + offset_pos.0 < 0
                    || curr_node.pos.0 as isize + offset_pos.0 >= height as isize
                {
                    continue;
                }
                if curr_node.pos.1 as isize + offset_pos.1 < 0
                    || curr_node.pos.1 as isize + offset_pos.1 >= width as isize
                {
                    continue;
                }

                let new_pos = (
                    (curr_node.pos.0 as isize + offset_pos.0) as usize,
                    (curr_node.pos.1 as isize + offset_pos.1) as usize,
                );
                let cost = grid[new_pos.0][new_pos.1];

                result.push((
                    Node::new(
                        new_pos,
                        *direction,
                        if curr_node.last_direction == *direction {
                            curr_node.last_direction_count + 1
                        } else {
                            1
                        },
                    ),
                    cost,
                ));
            }
            result
        },
        |curr_node| curr_node.pos == target && curr_node.last_direction_count >= 4,
    );

    if let Some((path, cost)) = path {
        // println!("{path:?}");
        println!("cost: {cost}");
    } else {
        println!("Failed to find path");
    }

}

fn main() {
    part2("/home/kali/projects/aoc/rust23/day17/part1/src/testcase.txt");
}
