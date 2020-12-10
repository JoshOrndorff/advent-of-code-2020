
use crate::{Aoc, AocBuilder};
use std::collections::HashSet;
use std::convert::TryInto;

pub struct Day8 {
    /// The parsed input program
    program: Vec<Instruction>,
}

#[derive(Debug)]
pub struct Machine {
    /// The parsed program that we're executing
    program: Vec<Instruction>,
    /// The instruction we're currently on
    pc: isize,
    /// The accumulator value
    acc: isize,
}

impl Machine {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            pc: 0,
            acc: 0,
        }
    }

    /// Perform one step of execution, mutating the current instance
    fn step(&mut self) {
        match self.program[self.pc as usize] {
            (Opcode::Nop, _) => {
                self.pc += 1;
            },
            (Opcode::Acc, arg) => {
                self.acc += arg;
                self.pc += 1;
            }
            (Opcode::Jmp, arg) => {
                self.pc += arg;
            }
        }
    }

    /// Execute the current instance until either an infinite loop occurs or the program exits normally
    /// Return the exit reason and the accumulator value
    fn execute(&mut self) -> (ExitReason, isize) {
        let mut visited_instructions = HashSet::new();
        let target_end_pc = self.program.len();
        while self.pc != target_end_pc.try_into().unwrap() {

            if visited_instructions.contains(&self.pc) {
                return (ExitReason::InfiniteLoop, self.acc);
            }

            visited_instructions.insert(self.pc);

            // println!("before step PC: {}, ACC: {}", machine.pc, machine.acc);
            self.step();
            // println!(" after step PC: {}, ACC: {}", machine.pc, machine.acc);
        }

        (ExitReason::Valid, self.acc)
    }
}

enum ExitReason {
    InfiniteLoop,
    Valid,
}

impl Aoc for Day8 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        let mut machine = Machine::new(self.program.to_vec());

        match machine.execute() {
            (ExitReason::InfiniteLoop, acc) => acc.to_string(),
            (ExitReason::Valid, _) => panic!("part one is expected to have an infinite loop"),
        }

    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        let mut hacked_program = self.program.clone();
        // Loop through all valid mutations of the given program.
        for i in 0..self.program.len() {
            // Make the mutation
            match self.program[i] {
                (Opcode::Acc, _) => {
                    continue;
                },
                (Opcode::Nop, arg) => {
                    hacked_program[i] = (Opcode::Jmp, arg);
                },
                (Opcode::Jmp, arg) => {
                    hacked_program[i] = (Opcode::Nop, arg);
                }
            }

            // See if the hack fixes the program.
            // If it does, return early, otherwise, undo the mutation for the next try
            let mut machine = Machine::new(hacked_program.to_vec());
            match machine.execute() {
                (ExitReason::Valid, acc) => {
                    return acc.to_string();
                },
                (ExitReason::InfiniteLoop, _) => {
                    hacked_program[i] = self.program[i].clone();
                },
            }
        }
        panic!("Tried all mutations; didn't find a valid one.");
    }
}

impl AocBuilder for Day8 {
    fn new(input: &String) -> Result<Self, &str> {

        Ok(Self {
            program: input.trim().lines().map(parse).collect(),
        })
    }
}

#[derive(Clone, Debug)]
enum Opcode {
    Acc,
    Jmp,
    Nop,
}

impl Opcode {
    fn from_str(s: &str) -> Self {
        match s {
            "acc" => Opcode::Acc,
            "jmp" => Opcode::Jmp,
            "nop"=> Opcode::Nop,
            _ => panic!("invalid opcode")
        }
    }
}

type Instruction = (Opcode, isize);

/// Parses source code into an instruction
fn parse(line: &str) -> Instruction {
    let space_index = line.find(" ").unwrap();

    (
        Opcode::from_str(&line[..space_index]),
        isize::from_str_radix(&line[space_index + 1..], 10).expect("valid isize"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_program() -> Day8 {
        Day8::new(&"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
".to_string()).expect("Example input is valid")
    }

    #[test]
    fn test_one() {
        assert_eq!(sample_program().solve_part_1(), "5");
    }
}
