use crate::{Aoc, AocBuilder};

/// Solver for Day 1
pub struct Day1 {
    expenses: Vec<u32>
}

impl Aoc for Day1 {
    /// Find the two expenses that sum to 2020 and return their product as a string
    fn solve_part_1(&self) -> String {
        for i in &self.expenses {
            for j in &self.expenses {
                if i + j == 2020 {
                    return (i * j).to_string();
                }
            }
        }
        panic!("Error solving day 1 part 1. Iterated all expenses pairwise; none summed to 2020");
    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        for i in &self.expenses {
            for j in &self.expenses {
                for k in &self.expenses {
                    if i + j + k == 2020 {
                        return (i * j * k).to_string();
                    }
                }
            }
        }
        panic!("Error solving day 1 part 2. Iterated all expenses triplet-wise; none summed to 2020");
    }
}

impl AocBuilder for Day1 {
    fn new(input: &String) -> Result<Self, &str> {
        Ok(
            Self {
                expenses: input.lines().map(|l| u32::from_str_radix(l, 10).expect("input should be valid")).collect()
            }
        )
    }
}
