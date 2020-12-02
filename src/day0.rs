use crate::AOC;

/// An example solver to make sure the cli works
/// The example problem is as follows
/// A page contains three numbers in the form (a, b), (c, d).
/// Part 1: Sum all four numbers
/// Part 2: calculate ac + bd
pub struct Day0 {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
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
