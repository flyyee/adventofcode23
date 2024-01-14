use itertools::Itertools;
use std::{collections::HashMap, collections::HashSet, fs, hash::Hash};

type BrickPart = (isize, isize, isize);
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Brick(BrickPart, BrickPart);

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, height: isize) -> usize {
        self.nodes.push(Node::new(height));
        self.nodes.len() - 1
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.nodes[from].children.push(to);
        self.nodes[to].parents.push(from);
    }

    pub fn chain(&self, root: usize) -> usize {
        let mut current_level_nodes = HashMap::<usize, isize>::new(); // node_idx => height_left
        current_level_nodes.insert(root, self.nodes[root].height);
        let mut triggered = 0usize;

        while !current_level_nodes.is_empty() {
            let mut next_level_nodes = HashMap::<usize, isize>::new();
            for (current_level_node, height) in current_level_nodes.iter_mut() {
                *height -= 1;
                if *height == 0 {
                    next_level_nodes.extend(
                        self.nodes[*current_level_node]
                            .children
                            .iter()
                            .map(|child| (*child, self.nodes[*child].height)),
                    );
                }
            }

            next_level_nodes = next_level_nodes
                .into_iter()
                .filter(|(next_level_node, _)| {
                    self.nodes[*next_level_node]
                        .parents
                        .iter()
                        .all(|parent| current_level_nodes.contains_key(parent))
                })
                .collect();

            triggered += next_level_nodes.len();

            next_level_nodes.extend(
                current_level_nodes
                    .into_iter()
                    .filter(|(_, height)| *height > 0),
            );

            current_level_nodes = next_level_nodes;
        }

        triggered
    }

    pub fn chain_all(&self) -> usize {
        (0..self.nodes.len()).map(|node| self.chain(node)).sum()
    }
}

struct Node {
    height: isize,
    children: Vec<usize>,
    parents: Vec<usize>,
}

impl Node {
    pub fn new(height: isize) -> Self {
        Self {
            height,
            children: Vec::new(),
            parents: Vec::new(),
        }
    }
}

struct BrickManager {
    tower: HashMap<(isize, isize), (isize, usize)>, // (x, y) => (highest z seen, brick_idx)
    graph: Graph,
}

impl BrickManager {
    pub fn new() -> Self {
        Self {
            tower: HashMap::new(),
            graph: Graph::new(),
        }
    }

    pub fn add_brick(&mut self, brick: &Brick) {
        let mut highest_z: Option<isize> = None;
        let mut supporting_bricks = HashSet::<usize>::new();
        let brick_height = brick.1 .2 - brick.0 .2 + 1;
        let node_idx = self.graph.add_node(brick_height);

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

        for supporting_brick in supporting_bricks.iter() {
            self.graph.add_edge(*supporting_brick, node_idx);
        }

        let my_z = highest_z.unwrap_or(0) + 1 + brick.1 .2 - brick.0 .2;
        for x in brick.0 .0..=brick.1 .0 {
            for y in brick.0 .1..=brick.1 .1 {
                self.tower
                    .entry((x, y))
                    .and_modify(|(z, idx)| {
                        if my_z > *z {
                            *z = my_z;
                            *idx = node_idx;
                        }
                    })
                    .or_insert((my_z, node_idx));
            }
        }
    }

    pub fn get_answer(&self) -> usize {
        self.graph.chain_all()
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
