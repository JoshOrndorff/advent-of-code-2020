use crate::{Aoc, AocBuilder};


type Instruction = (char, isize);

/// The four cardinal directions
enum Cardinal {
    N,
    S,
    E,
    W,
}

pub struct Day12 {
    instructions: Vec<Instruction>
}

struct Ship {
    // North is positive
    ns: isize,
    // East is positive
    ew: isize,
    facing: Cardinal
}

impl Ship {
    fn new() -> Self {
        Self {
            ns: 0,
            ew: 0,
            facing: Cardinal::E,
        }
    }

    fn step(&mut self, inst: &Instruction) {
        let (c, val) = inst;

        match c {
            'N' => {
                self.ns += val;
            },
            'S' => {
                self.ns -= val;
            },
            'E' => {
                self.ew += val;
            },
            'W' => {
                self.ew -= val;
            },
            'L' => {
                let right_angles = val / 90;
                for _ in 0..right_angles {
                    match self.facing {
                        Cardinal::N => { self.facing = Cardinal::W },
                        Cardinal::W => { self.facing = Cardinal::S },
                        Cardinal::S => { self.facing = Cardinal::E },
                        Cardinal::E => { self.facing = Cardinal::N },
                    }
                }
            },
            'R' => {
                let right_angles = val / 90;
                for _ in 0..right_angles {
                    match self.facing {
                        Cardinal::N => { self.facing = Cardinal::E },
                        Cardinal::E => { self.facing = Cardinal::S },
                        Cardinal::S => { self.facing = Cardinal::W },
                        Cardinal::W => { self.facing = Cardinal::N },
                    }
                }
            },
            'F' => {
                match self.facing {
                    Cardinal::N => { self.ns += val },
                    Cardinal::S => { self.ns -= val },
                    Cardinal::E => { self.ew += val },
                    Cardinal::W => { self.ew -= val },
                }
            },
            c => {
                panic!("Invalid instruction character encounteres : {}", c);
            }
        }
    }
}

impl Aoc for Day12 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        let mut ship = Ship::new();
        for instr in &self.instructions {
            ship.step(instr);
        }

        let manhattan_distance = ship.ns.abs() + ship.ew.abs();

        manhattan_distance.to_string()
    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        todo!()
    }
}

impl AocBuilder for Day12 {
    fn new(input: &String) -> Result<Self, &str> {

        Ok(Self {
            instructions: input.trim().lines().map(parse).collect(),
        })
    }
}

/// Parses one line of string input into useable data
/// TODO redo this with nom
fn parse(line: &str) -> Instruction {
    let c = line.chars().next().expect("At least one character exists (the letter instruction)");
    let val = isize::from_str_radix(&line[1..line.len()], 10).expect("Valid input numbers");

    (c, val)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Day12 {
        Day12::new(&"F10\nN3\nF7\nR90\nF11\n".to_string()).expect("example input is valid")
    }

    #[test]
    fn test_one() {
        assert_eq!(example_input().solve_part_1(), "25");
    }
}
