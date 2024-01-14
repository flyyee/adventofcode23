use std::cmp;
use std::fs;

fn main() {
    let input =
        fs::read_to_string("/home/kali/projects/aoc/rust23/day1/trebuchet/src/testcase.txt")
            .expect("Failed to read input file");
    println!("{}", input);
    const NUMBERS: [&str; 20] = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];
    let sum: u32 = input
        .split_whitespace()
        .map(|x| {
            // let first_digit = x
            //     .chars()
            //     .filter(|x| x.is_numeric())
            //     .nth(0)
            //     .expect("No digits in the number");
            // let last_digit = x
            //     .chars()
            //     .rev()
            //     .filter(|x| x.is_numeric())
            //     .nth(0)
            //     .expect("No digits in the number");
            let mut begin = x.len();
            let mut end: usize = 0;

            let mut first_digit: u32 = 0;
            let mut last_digit: u32 = 0;

            for (idx, &number) in NUMBERS.iter().enumerate() {
                if let Some(pos) = x.find(number) {
                    if begin > pos {
                        begin = pos;
                        first_digit = (idx % 10) as u32;
                    }
                }
                if let Some(pos) = x.rfind(number) {
                    if end <= pos {
                        end = pos;
                        last_digit = (idx % 10) as u32;
                    }
                }
            }

            first_digit * 10 + last_digit

            // let first_digit = x.find(|slice| false);
            // first_digit.to_digit(10).expect("Digit is not radix 10") * 10
            //     + last_digit.to_digit(10).expect("Digit is not radix 10")
        })
        .sum();
    println!("{}", sum);
}
