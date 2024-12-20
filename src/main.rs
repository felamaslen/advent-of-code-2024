use clap::Parser;
use std::fs::File;
use std::io::prelude::*;

use day1::day1;
use day10::day10;
use day11::day11;
use day12::day12;
use day13::day13;
use day14::{day14, Day14Input};
use day15::day15;
use day16::day16;
use day17::day17;
use day18::day18;
use day2::day2;
use day3::day3;
use day4::day4;
use day5::day5;
use day6::day6;
use day7::day7;
use day8::day8;
use day9::day9;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
        4 => {
            let mut input = String::new();
            let mut file = File::open("src/day4_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day4(input);
            println!("Day 4 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        5 => {
            let mut input = String::new();
            let mut file = File::open("src/day5_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day5(input);
            println!("Day 5 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        6 => {
            let mut input = String::new();
            let mut file = File::open("src/day6_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day6(input);
            println!("Day 6 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        7 => {
            let mut input = String::new();
            let mut file = File::open("src/day7_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day7(input);
            println!("Day 7 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        8 => {
            let mut input = String::new();
            let mut file = File::open("src/day8_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day8(input);
            println!("Day 8 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        9 => {
            let mut input = String::new();
            let mut file = File::open("src/day9_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day9(input);
            println!("Day 9 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        10 => {
            let mut input = String::new();
            let mut file = File::open("src/day10_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day10(input);
            println!("Day 10 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        11 => {
            let mut input = String::new();
            let mut file = File::open("src/day11_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day11(input);
            println!("Day 11 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        12 => {
            let mut input = String::new();
            let mut file = File::open("src/day12_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day12(input);
            println!("Day 12 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        13 => {
            let mut input = String::new();
            let mut file = File::open("src/day13_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day13(input);
            println!("Day 13 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        14 => {
            let mut input = String::new();
            let mut file = File::open("src/day14_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day14(Day14Input {
                robots: &input,
                width: 101,
                height: 103,
            });
            println!("Day 14 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        15 => {
            let mut input = String::new();
            let mut file = File::open("src/day15_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day15(input.to_owned());
            println!("Day 15 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        16 => {
            let mut input = String::new();
            let mut file = File::open("src/day16_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day16(input.to_owned());
            println!("Day 16 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        17 => {
            let mut input = String::new();
            let mut file = File::open("src/day17_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day17(input.to_owned());
            println!("Day 17 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        18 => {
            let mut input = String::new();
            let mut file = File::open("src/day18_input.txt").expect("Error opening input");
            file.read_to_string(&mut input)
                .expect("Error reading input");
            let result = day18(day18::InputDay18 {
                bytes: input,
                grid_size: 71,
                falls: 1024,
            });
            println!("Day 18 result:");
            println!("Part 1: {}", result.part1);
            println!("Part 2: {}", result.part2);
        }
        _ => panic!("Unknown or unfinished day {}", cli.day),
    }
}
