use std::cmp;
use std::fs;

fn main() {
    println!("Hello, world!");
    let input =
        fs::read_to_string("/home/kali/projects/aoc/rust23/day2/cube_conundrum-1/src/testcase.txt")
            .expect("Failed to read input file");
    let mut ans = 0u32;
    'game_loop: for game in input.split("\n") {
        let game = game.split(": ").collect::<Vec<&str>>();
        let (prefix, suffix) = (game[0], game[1]);
        let game_id: u32 = prefix[5..].parse::<u32>().expect("Invalid game ID");
        let mut green_max = 0u32;
        let mut red_max = 0u32;
        let mut blue_max = 0u32;

        for revelation in suffix.split("; ") {
            for kind in revelation.split(", ") {
                let kind: Vec<&str> = kind.split(" ").collect();
                let (number, color) = (kind[0], kind[1]);
                let number = number.parse::<u32>().expect("Failed to pass number");
                match color {
                    "red" => {
                        red_max = cmp::max(red_max, number);
                    }
                    "green" => {
                        green_max = cmp::max(green_max, number);
                    }
                    "blue" => {
                        blue_max = cmp::max(blue_max, number);
                    }
                    _ => {
                        panic!("Failed to match color.")
                    }
                }
            }
        }

        ans += red_max * green_max * blue_max;
    }

    println!("{}", ans);
}
