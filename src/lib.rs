mod day0;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day15;

pub use day0::Day0;
pub use day1::Day1;
pub use day2::Day2;
pub use day3::Day3;
pub use day4::Day4;
pub use day5::Day5;
pub use day6::Day6;
pub use day7::Day7;
pub use day8::Day8;
pub use day9::Day9;
pub use day10::Day10;
pub use day11::Day11;
pub use day12::Day12;
pub use day15::Day15;

/// Something that can build an AOC instance
pub trait AocBuilder {
    fn new(_: &String) -> Result<Self, &str>
    where
        Self: Sized;
}

/// An instance of an AOC puzzle with methods to solve each part.
pub trait Aoc {
    // The type in which the answer will be returned.
    // I couldn't figure out how to do dynamic dispatch in this case.
    //type Solution: Display;

    /// Get the solution to part 1
    fn solve_part_1(&self) -> String;
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String;
}
