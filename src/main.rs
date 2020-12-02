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

    let solver = get_solver(params.day, &input).expect(&format!("Invalid day specified: {}", params.day));

    println!("Solving day {} with input: {}", params.day, &input);

    println!("Part 1 solution: {}", solver.solve_part_1());
    println!("Part 2 solution: {}", solver.solve_part_2());
}

fn get_solver(day: u8, input: &String) -> Result<Box<dyn AOC>, ()> {

    Ok(
        match day {
            0 => Box::new(day0::Day0{ a: 1, b: 2, c: 3, d: 4 }) as Box<dyn AOC>,
            1 => Box::new(day1::Day1{
                expenses: input.lines().map(|l| u32::from_str_radix(l, 10).expect("input should be valid")).collect()
            })as Box<dyn AOC>,
            _ => Err(())?
        }
    )
}

// Maybe this is the long term solution to having the parsing happen in the individual days
pub trait AOCBuilder {
    fn new(_: &String) -> Box<dyn AOC>;
}

pub trait AOC {
    // The type in which the answer will be returned.
    //type Solution: Display;

    // Create a new instance of the solver with the given string input
    // I removed this because in order to do dynamic dispatch all functions have to be methods.
    // One option is to have a method called initialize for reinitialize or something.
    // but for now I'll just keep it simple.
    //fn new_with_input(_: &String) -> Self;
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String;
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String;
}
