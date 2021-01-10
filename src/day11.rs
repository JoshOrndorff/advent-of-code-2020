use crate::{Aoc, AocBuilder};


pub struct Day11 {
    room: Vec<Vec<State>>,
}

fn step_room(room: &[&[State]]) -> Vec<Vec<State>> {
    for i
}

fn

enum State {
    Floor,
    Empty,
    Occupied,
}

impl Aoc for Day11 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        todo!()
    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        todo!()
    }
}

impl AocBuilder for Day11 {
    fn new(input: &String) -> Result<Self, &str> {

        Ok(Self {
            room: input.trim().lines().map(|l| l.chars().map(parse).collect()).collect(),
        })
    }
}

/// Parses a single character into a State variant
fn parse(c: char) -> State {
    match c {
        'L' => State::Empty,
        '.' => State::Floor,
        '#' => State::Occupied,
        other => panic!("Invalid character: {}", other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert!(false);
    }
}
