use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

use day1::day1;
use day2::day2;
use day3::day3;

mod day1;
mod day2;
mod day3;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    day: i16,
}

fn main() {
    let cli = Cli::parse();

    match cli.day {
        1 => {
            let mut input = String::new();
            let mut file = File::open("src/day1_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day1(input);
            println!("Day 1 result:");
            println!("Part 1: {}", result.diff_sum);
            println!("Part 2: {}", result.similarity_score);
        }
        2 => {
            let mut input = String::new();
            let mut file = File::open("src/day2_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day2(input);
            println!("Day 2 result:");
            println!("Part 1: {}", result.num_safe);
            println!("Part 2: {}", result.num_safe_with_dampener);
        }
        3 => {
            let mut input = String::new();
            let mut file = File::open("src/day3_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day3(input);
            println!("Day 3 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        _ => panic!("Unknown or unfinished day {}", cli.day),
    }
}
