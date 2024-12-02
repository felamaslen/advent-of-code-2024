use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

use day1::day1;
use day2::day2;

mod day1;
mod day2;

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
            println!("Day 2 result: {}", result);
        }
        _ => panic!("Unknown or unfinished day {}", cli.day),
    }
}
