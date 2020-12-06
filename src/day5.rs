//! The seat codes are simply 10-bit binary numbers where B and R represent set bits.

// 1023 is too high
// 511 is too low

use crate::{Aoc, AocBuilder};

pub struct Day5 {
    seat_ids: Vec<u16>,
}

impl Aoc for Day5 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        self.seat_ids.iter().max().expect("At least one input supplied.").to_string()
    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {

        let mut sorted = self.seat_ids.clone();
        sorted.sort();

        let mut expected = sorted[0] - 1;

        for id in &sorted {
            expected += 1;
            if id != &expected {
                return expected.to_string()
            }
        }

        panic!("Got to end of list without finding missing seat id");
    }
}

impl AocBuilder for Day5 {
    fn new(input: &String) -> Result<Self, &str> {

        Ok(Self {
            seat_ids: input.lines().map(|l| seat_id(&l.to_string())).collect(),
        })
    }
}

fn seat_id(code: &str) -> u16 {
    let mut result = 0;
    for c in code.chars() {
        if c == 'B' || c == 'R' {
            result += 1;
        }
        result = result << 1;
    }

    // Shift once back to the right to cancel the last shift in the loop.
    result >> 1
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_one() {
        assert_eq!(seat_id(&"BFFFBBFRRR"), 567);
    }

    #[test]
    fn parse_two() {
        assert_eq!(seat_id(&"FFFBBBFRRR"), 119);
    }

    #[test]
    fn parse_three() {
        assert_eq!(seat_id(&"BBFFBBFRLL"), 820);
    }
}
