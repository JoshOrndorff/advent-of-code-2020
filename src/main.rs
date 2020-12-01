use std::fs;
use structopt::StructOpt;

mod day0;
mod day1;

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
        (None, None) => panic!("Must supply input on cli or via file")
    };

    println!("Solving day {} with input: {}", params.day, input);

    let (solution1, solution2) = match params.day {
        0 => {
            let solver = day0::Day0::new_with_input(&input);
            (solver.solve_part_1(), solver.solve_part_2())
        }
        1 => {
            let solver = day1::Day1::new_with_input(&input);
            (solver.solve_part_1(), solver.solve_part_2())
        }
        _ => panic!("invalid day supplied"),
    };

    println!("Part 1 solution: {}", solution1);
    println!("Part 2 solution: {}", solution2);
}

pub trait AOC {
    /// Create a new instance of the solver with the given string input
    fn new_with_input(_: &String) -> Self;
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String;
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String;
}
