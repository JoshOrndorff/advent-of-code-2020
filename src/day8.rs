
use crate::{Aoc, AocBuilder};
use std::collections::HashSet;


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
}

impl Aoc for Day8 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        let mut machine = Machine::new(self.program.to_vec());

        let mut visited_instructions = HashSet::new();
        while !visited_instructions.contains(&machine.pc) {
            visited_instructions.insert(machine.pc);
            // println!("before step PC: {}, ACC: {}", machine.pc, machine.acc);
            machine.step();

            // println!(" after step PC: {}, ACC: {}", machine.pc, machine.acc);

        }

        machine.acc.to_string()

    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        todo!()
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
