use crate::{Aoc, AocBuilder};
use std::fmt::{self, Display};
use std::convert::TryInto;
use std::hash::Hash;
use std::collections::hash_map::DefaultHasher;

#[derive(Eq, PartialEq, Hash)]
pub struct Day11 {
    room: Vec<Vec<State>>,
    num_rows: usize,
    num_cols: usize,
}

impl Display for Day11 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.room {
            for spot in row {
                write!(f, "{}", spot)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
enum State {
    Floor,
    Empty,
    Occupied,
}

impl Day11 {

    // Returns whether a cell is occupied or not.
    // Coordinates are signed to allow passing number just outside of the left and top bound.
    // The cell is considered unoccupied (we return false) if it is empty, floor, or nonexistant
    fn occupied(&self, i: isize, j: isize) -> bool {
        // Shadow the parameters with unsigned versions
        let i: usize = match i.try_into() {
            Ok(i) => i,
            Err(_) => return false,
        };

        let j: usize = match j.try_into() {
            Ok(j) => j,
            Err(_) => return false,
        };
        if i > self.room.len() || j > self.room.len() {
            return false;
        }

        self.room[i as usize][j as usize] == State::Occupied
    }

    // Count the number of cells adjascent to this one that are occupied. [0, 8]
    fn occupied_adjascent(&self, i: usize, j: usize) -> u8 {

        // Calculate the bounds of the adjascent cells to check
        let i_lower = if i == 0 { 0 } else {i - 1};
        let i_upper = if i == self.num_rows - 1 {i} else {i + 1};
        let j_lower = if j == 0 { 0 } else {j - 1};
        let j_upper = if j == self.num_cols - 1 {j} else {j + 1};

        let mut occupied_adjascent = 0;

        for x in i_lower..i_upper {
            for y in j_lower..j_upper {
                // Careful not to count the current cell, just the neighbors
                if x != y && self.room[x][y] == State::Occupied {
                    occupied_adjascent += 1;
                }
            }
        }

        occupied_adjascent
    }

    fn next_cell(&self, i: usize, j: usize) -> State {
        match &self.room[i][j] {
            State::Empty if self.occupied_adjascent(i, j) == 0 => State::Occupied,
            State::Occupied if self.occupied_adjascent(i, j) >= 4 => State::Empty,
            other => other.clone() //TODO Probably all of the Vecs could hold references...
        }
    }

    fn next_room(&self) -> Day11 {
        let mut new_room: Vec<Vec<_>> = Vec::new();

        for i in 0..self.num_rows {
            let mut row: Vec<_> = Vec::new();

            for j in 0..self.num_cols {
                row.push(self.next_cell(i, j));
            }
            new_room.push(row);
        }

        Day11 {
            room: new_room,
            num_rows: self.num_rows,
            num_cols: self.num_cols,
        }
    }
}

// https://users.rust-lang.org/t/how-can-i-implement-fmt-display-for-enum/24111/3
impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            State::Floor => write!(f, "."),
            State::Empty => write!(f, "L"),
            State::Occupied => write!(f, "#"),
        }
    }
}

impl Aoc for Day11 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        let mut current_board = self;
        let mut previous_hash = self.hash(&mut DefaultHasher::new());

        for i in 0..4 {
            println!("{}", current_board);
            let next_board = current_board.next_room();
            let next_hash = next_board.hash(&mut DefaultHasher::new());
            if &next_hash == previous_hash {
                println!("Found repeat board");
                break;
            }
            current_board = &next_board;
        }
        
        //
        "hi".into()
    }

    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        todo!()
    }
}

impl AocBuilder for Day11 {
    fn new(input: &String) -> Result<Self, &str> {

        let room: Vec<Vec<_>> = input.trim().lines().map(|l| l.chars().map(parse).collect()).collect();
        let num_rows = room.len();
        let num_cols = room[0].len();

        Ok(Self {
            room,
            num_rows,
            num_cols,
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

    fn example_input() -> Day11 {
        Day11::new(&include_str!("../inputs/11example.txt").to_string()).expect("example input is valid")
    }

    #[test]
    fn example_starts_completely_unoccupied() {
        let example = example_input();
        for i in 0..10 {
            for j in 0..10 {
                assert_eq!(example.occupied_adjascent(i, j), 0);
            }
        }
    }
}
