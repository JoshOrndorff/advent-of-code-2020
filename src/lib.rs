
mod day0;
mod day1;
mod day2;

pub use day0::Day0;
pub use day1::Day1;
pub use day2::Day2;

/// Something that can build an AOC instance
pub trait AocBuilder {
    fn new(_: &String) -> Result<Self, &str> where Self: Sized;
}

/// An instance of an AOC puzzle with methods to solve each part.
pub trait Aoc {
    // The type in which the answer will be returned.
    //type Solution: Display;

    /// Get the solution to part 1
    fn solve_part_1(&self) -> String;
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String;
}