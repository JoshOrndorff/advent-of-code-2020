use crate::{AOC, AOCBuilder};

/// An example solver to make sure the cli works
/// The example problem is as follows
/// A page contains three numbers in the form (a, b), (c, d).
/// Part 1: Sum all four numbers
/// Part 2: calculate ac + bd
pub struct Day0 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

impl AOC for Day0 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        format!("{}", self.a + self.b + self.c + self.d)
    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        todo!()
    }
}

impl AOCBuilder for Day0 {
    fn new(_: &String) -> Box<dyn AOC> {
        //TODO actual parser
        Box::new(Self {a: 0, b: 1, c: 2, d: 4}) as Box<dyn AOC>
    }
}
