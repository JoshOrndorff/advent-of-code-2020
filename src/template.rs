use crate::{Aoc, AocBuilder};


pub struct Day3 {

}

impl Aoc for Day3 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        todo!()
    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        todo!()
    }
}

impl AocBuilder for Day3 {
    fn new(input: &String) -> Result<Self, &str> {

        Ok(Self {})
    }
}

/// Parses one line of string input into useable data
/// TODO redo this with nom
fn parse(line: &str) -> Result<(), &str> {
    // let colon_index = line
    //     .find(":")
    //     .ok_or("each line contains exactly one colon")?;
    // let num = u8::from_str_radix(&line[..colon_index], 10);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert!(false);
    }
}
