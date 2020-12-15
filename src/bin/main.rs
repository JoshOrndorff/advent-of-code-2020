extern crate advent_of_code_2020;

use advent_of_code_2020::*;
use std::fs;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct CLI {
    /// Which day to solve
    #[structopt(short, long, default_value = "1")]
    day: u8,
    /// Path to the file the input should be read from
    #[structopt(long)]
    input_file: Option<String>,
    /// Input to be used directly
    #[structopt(long)]
    input: Option<String>,
}

fn main() {
    let params = CLI::from_args();

    let input = match (params.input, params.input_file) {
        (Some(input), _) => input,
        (None, Some(path)) => fs::read_to_string(path).expect("Input file not read successfully"),
        (None, None) => panic!("Must supply input on cli or via file"),
    };

    let solver =
        get_solver(params.day, &input).expect(&format!("Invalid day specified: {}", params.day));

    println!("Solving day {}", params.day);

    println!("Part 1 solution: {}", solver.solve_part_1());
    println!("Part 2 solution: {}", solver.solve_part_2());
}

fn get_solver(day: u8, input: &String) -> Result<Box<dyn Aoc>, &str> {
    Ok(match day {
        0 => Box::new(Day0::new(input)?) as Box<dyn Aoc>,
        1 => Box::new(Day1::new(input)?) as Box<dyn Aoc>,
        2 => Box::new(Day2::new(input)?) as Box<dyn Aoc>,
        3 => Box::new(Day3::new(input)?) as Box<dyn Aoc>,
        4 => Box::new(Day4::new(input)?) as Box<dyn Aoc>,
        5 => Box::new(Day5::new(input)?) as Box<dyn Aoc>,
        6 => Box::new(Day6::new(input)?) as Box<dyn Aoc>,
        7 => Box::new(Day7::new(input)?) as Box<dyn Aoc>,
        8 => Box::new(Day8::new(input)?) as Box<dyn Aoc>,
        9 => Box::new(Day9::new(input)?) as Box<dyn Aoc>,
        10 => Box::new(Day10::new(input)?) as Box<dyn Aoc>,
        // 11 => Box::new(Day11::new(input)?) as Box<dyn Aoc>,
        12 => Box::new(Day12::new(input)?) as Box<dyn Aoc>,
        15 => Box::new(Day15::new(input)?) as Box<dyn Aoc>,
        _ => Err("invalid day specified")?,
    })
}
