use num_bigint::BigUint;
use std::{cmp::Ordering, collections::HashMap, fs, mem};

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
struct Bound(usize, usize);

impl Bound {
    pub fn split_lt(&mut self, value: usize) -> Bound {
        // Returns a bound derived from self that satisfies the lt ordering
        // Modifies self to be the remainder of the split
        if value <= self.0 {
            return Bound(0, 0);
        } else if value > (self.0 + self.1 - 1) {
            return mem::take(self);
        }
        let ret = Bound(self.0, value - self.0);
        self.1 -= value - self.0;
        self.0 = value;
        ret
    }

    pub fn split_gt(&mut self, value: usize) -> Bound {
        // Returns a bound derived from self that satisfies the gt ordering
        // Modifies self to be the remainder of the split
        if value >= self.0 + self.1 - 1 {
            return Bound(0, 0);
        } else if value < self.0 {
            return mem::take(self);
        }
        let ret = Bound(value + 1, self.0 + self.1 - (value + 1));
        self.1 = value - self.0 + 1;
        ret
    }

    pub fn empty(&self) -> bool {
        self.1 == 0
    }
}

#[derive(Clone, Copy)]
struct Part {
    x: Bound,
    m: Bound,
    a: Bound,
    s: Bound,
}

impl Part {
    pub fn apply(&mut self, s_register: &str, operator: Ordering, operand: usize) -> Part {
        let mut sat_part = *self;
        let register = match s_register {
            "x" => &mut self.x,
            "m" => &mut self.m,
            "a" => &mut self.a,
            "s" => &mut self.s,
            _ => unreachable!(),
        };
        let sat_bound = match operator {
            Ordering::Greater => register.split_gt(operand),
            Ordering::Less => register.split_lt(operand),
            _ => unreachable!(),
        };
        match s_register {
            "x" => sat_part.x = sat_bound,
            "m" => sat_part.m = sat_bound,
            "a" => sat_part.a = sat_bound,
            "s" => sat_part.s = sat_bound,
            _ => unreachable!(),
        };

        sat_part
    }

    pub fn empty(&self) -> bool {
        self.x.empty() || self.m.empty() || self.a.empty() || self.s.empty()
    }

    pub fn ways(&self) -> BigUint {
        BigUint::from(self.x.1)
            * BigUint::from(self.m.1)
            * BigUint::from(self.a.1)
            * BigUint::from(self.s.1)
    }
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

    pub fn run(
        &self,
        part: &mut Part,
        workflows: &HashMap<String, Workflow>,
        counter: &mut BigUint,
    ) {
        for rule in &self.rules {
            if let Rule::Condition(condition) = rule {
                let mut satisfies = part.apply(
                    &condition.register[..],
                    condition.operator,
                    condition.operand,
                );
                if !satisfies.empty() {
                    workflows[&condition.destination].run(&mut satisfies, workflows, counter);
                }
            } else if let Rule::Jump(jump) = rule {
                workflows[&jump.destination].run(part, workflows, counter);
            } else if let Rule::Return(value) = rule {
                if matches!(value, WorkflowResult::Accepted) {
                    *counter += part.ways();
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum WorkflowResult {
    Rejected,
    Accepted,
}

fn part2(filename: &str) {
    let input = fs::read_to_string(filename).unwrap();
    let (workflows, _) = input.split_once("\n\n").unwrap();
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
        .chain(vec![
            (
                String::from("A"),
                Workflow::new(vec![Rule::Return(WorkflowResult::Accepted)]),
            ),
            (
                String::from("R"),
                Workflow::new(vec![Rule::Return(WorkflowResult::Rejected)]),
            ),
        ])
        .collect();

    let mut part = Part {
        x: Bound(1, 4000),
        m: Bound(1, 4000),
        a: Bound(1, 4000),
        s: Bound(1, 4000),
    };

    let mut counter: BigUint = BigUint::from(0usize);

    workflows["in"].run(&mut part, &workflows, &mut counter);

    println!("counter = {counter:?}");
}

fn main() {
    part2("/home/kali/projects/aoc/rust23/day19/part1/src/testcase.txt");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bounds_lt() {
        use crate::Bound;
        let mut bound = Bound(1, 4000);
        let sat_bound = bound.split_lt(400);
        assert_eq!(sat_bound, Bound(1, 399));
        assert_eq!(bound, Bound(400, 3601));

        let sat_bound = bound.split_lt(400);
        assert_eq!(sat_bound, Bound(0, 0));
        assert_eq!(bound, Bound(400, 3601));

        let sat_bound = bound.split_lt(4001);
        assert_eq!(sat_bound, Bound(400, 3601));
        assert_eq!(bound, Bound(0, 0));
    }

    #[test]
    fn test_bounds_gt() {
        use crate::Bound;
        let mut bound = Bound(1, 4000);
        let sat_bound = bound.split_gt(3600);
        assert_eq!(sat_bound, Bound(3601, 400));
        assert_eq!(bound, Bound(1, 3600));

        let sat_bound = bound.split_gt(3600);
        assert_eq!(sat_bound, Bound(0, 0));
        assert_eq!(bound, Bound(1, 3600));

        let sat_bound = bound.split_gt(0);
        assert_eq!(sat_bound, Bound(1, 3600));
        assert_eq!(bound, Bound(0, 0));
    }
}
