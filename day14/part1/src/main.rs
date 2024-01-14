use std::fs;

fn calculate_column<'a>(column: impl Iterator<Item = &'a u8>, height: usize) -> usize {
    let mut score = 0usize;
    let mut prev_square: Option<usize> = None;
    let mut circles = 0usize;
    for (i, element) in column.enumerate() {
        if *element == b'O' {
            circles += 1;
        } else if *element == b'#' {
            let square_pos = if let Some(val) = prev_square {
                height - val
            } else {
                height + 1
            };

            score += circles * square_pos - (circles * (circles + 1) / 2);

            prev_square = Some(i);
            circles = 0;
        }
    }

    if circles != 0 {
        let square_pos = if let Some(val) = prev_square {
            height - val
        } else {
            height + 1
        };
        score += circles * square_pos - (circles * (circles + 1) / 2);
    }

    score
}

fn main() {
    let input = fs::read_to_string("/home/kali/projects/aoc/rust23/day14/part1/src/testcase.txt")
        .unwrap()
        .trim()
        .to_owned();

    let width = input.find('\n').unwrap();
    let height = (input.len() + 1) / (width + 1);
    let mut ans = 0;

    for i in 0..width {
        ans += calculate_column(input.as_bytes().iter().skip(i).step_by(width + 1), height);
    }

    println!("{ans}");
}
