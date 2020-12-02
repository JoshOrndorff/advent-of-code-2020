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
            0 => day0::Day0::new(input),
            1 => day1::Day1::new(input),
            _ => Err(())?
        }
    )
}

/// Something that can build an AOC instance
pub trait AOCBuilder {
    fn new(_: &String) -> Box<dyn AOC>;
}

/// An instance of an AOC puzzle with methods to solve each part.
pub trait AOC {
    // The type in which the answer will be returned.
    //type Solution: Display;

    /// Get the solution to part 1
    fn solve_part_1(&self) -> String;
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String;
}
