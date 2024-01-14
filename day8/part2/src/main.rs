use std::collections::HashMap;
use std::fs;

fn main() {
    let input =
        fs::read_to_string("/home/kali/projects/aoc/rust23/day8/part2/src/testcase.txt").unwrap();
    let instructions = input.split('\n').nth(0).unwrap();

    let directions: HashMap<String, (String, String)> = input
        .split('\n')
        .skip(2)
        .map(|x| {
            let lhs = x[0..3].to_owned();
            let rhs1 = x[7..10].to_owned();
            let rhs2 = x[12..15].to_owned();
            (lhs, (rhs1, rhs2))
        })
        .collect();

    let mut currs = directions
        .iter()
        .filter_map(|(x, _)| {
            if x.ends_with("A") {
                Some(x.as_str())
            } else {
                None
            }
        })
        .collect::<Vec<&str>>();

    let mut all_steps = Vec::<usize>::new();
    for curr in currs.iter_mut() {
        let mut steps = 0usize;
        for instruction in instructions.as_bytes().iter().cycle() {
            match &instruction {
                b'L' => *curr = directions[*curr].0.as_str(),
                b'R' => *curr = directions[*curr].1.as_str(),
                _ => panic!("fail match"),
            }

            steps += 1;
            if curr.ends_with("Z") {
                break;
            }
        }
        // println!("{steps}");
        all_steps.push(steps);
    }

    pub fn lcm(nums: &[usize]) -> usize {
        if nums.len() == 1 {
            return nums[0];
        }
        let a = nums[0];
        let b = lcm(&nums[1..]);
        a * b / gcd_of_two_numbers(a, b)
    }

    fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        }
        gcd_of_two_numbers(b, a % b)
    }

    let ans = lcm(all_steps.as_slice());
    println!("{}", ans);

    // TODO: use lcm optimization
}
