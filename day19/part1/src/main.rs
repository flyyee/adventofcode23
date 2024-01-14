use std::{cmp::Ordering, collections::HashMap, fs};

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

struct Condition {
    register: String,
    operator: Ordering,
    operand: usize,
    destination: String,
}

struct Jump {
    destination: String,
}

enum Rule {
    Condition(Condition),
    Jump(Jump),
    Return(WorkflowResult),
}

struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    pub fn run(&self, part: &Part, workflows: &HashMap<String, Workflow>) -> WorkflowResult {
        for rule in &self.rules {
            if let Rule::Condition(condition) = rule {
                let register = match &condition.register[..] {
                    "x" => part.x,
                    "m" => part.m,
                    "a" => part.a,
                    "s" => part.s,
                    _ => unreachable!(),
                };
                if register.cmp(&condition.operand) == condition.operator {
                    return workflows[&condition.destination].run(part, workflows);
                }
            } else if let Rule::Jump(jump) = rule {
                return workflows[&jump.destination].run(part, workflows);
            } else if let Rule::Return(value) = rule {
                return *value;
            }
        }
        unreachable!()
    }
}

#[derive(Clone, Copy)]
enum WorkflowResult {
    Rejected,
    Accepted,
}

fn part1(filename: &str) {
    let input = fs::read_to_string(filename).unwrap();
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows: HashMap<String, Workflow> = workflows
        .split('\n')
        .map(|s| {
            let (name, mut rules) = s.split_once('{').unwrap();
            rules = &rules[..rules.len() - 1];
            let rules = rules
                .split(',')
                .map(|s| {
                    if !s.contains(':') {
                        Rule::Jump(Jump {
                            destination: s.to_owned(),
                        })
                    } else {
                        let delim_pos = s.find(':').unwrap();
                        Rule::Condition(Condition {
                            register: s[..1].to_owned(),
                            operator: match s.chars().nth(1).unwrap() {
                                '>' => Ordering::Greater,
                                '<' => Ordering::Less,
                                _ => unreachable!(),
                            },
                            operand: s[2..delim_pos].parse().unwrap(),
                            destination: s[delim_pos + 1..].to_owned(),
                        })
                    }
                })
                .collect::<Vec<Rule>>();
            (name.to_owned(), Workflow::new(rules))
        })
        .chain(
            vec![
                (
                    String::from("A"),
                    Workflow::new(vec![Rule::Return(WorkflowResult::Accepted)]),
                ),
                (
                    String::from("R"),
                    Workflow::new(vec![Rule::Return(WorkflowResult::Rejected)]),
                ),
            ]
            .into_iter(),
        )
        .collect();

    let parts: Vec<Part> = parts
        .split('\n')
        .map(|part| {
            let part = &part[..part.len() - 1];
            let part_values: Vec<usize> = part
                .split(',')
                .map(|s| s.split_once('=').unwrap().1.parse().unwrap())
                .collect();
            Part {
                x: part_values[0],
                m: part_values[1],
                a: part_values[2],
                s: part_values[3],
            }
        })
        .collect();

    let ans: usize = parts
        .iter()
        .filter(|&part| {
            matches!(
                workflows["in"].run(part, &workflows),
                WorkflowResult::Accepted
            )
        })
        .map(|part| part.x + part.m + part.a + part.s)
        .sum();

    println!("{ans}");
}

fn main() {
    part1("/home/kali/projects/aoc/rust23/day19/part1/src/testcase.txt");
}
