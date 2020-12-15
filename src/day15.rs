use crate::{Aoc, AocBuilder};
use std::collections::HashMap;


pub struct Day15 {
    starting_numbers: Vec<usize>,
}

impl Aoc for Day15 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        // A map from numbers to when they were last spoken.
        let mut last_spoken : HashMap<usize, usize> = HashMap::new();

        // Initialize the hashmap with all but one starting number
        for turn in 0..self.starting_numbers.len() - 1 {
            last_spoken.insert(self.starting_numbers[turn], turn);
        }

        // Initialize the "previous" to the last starting number
        let mut previous: usize = *self.starting_numbers.iter().last().expect("There is a last");

        // Now start the interesting turns.
        // The problem is phrased in 1-based, but I'm using 0-based. We only report
        // relative ages, so I think it's fine. We're ending on turn number 2019 (0-based)
        for turn in self.starting_numbers.len()..2020 {
            let this_num = match last_spoken.get(&previous) {
                None => {
                    println!("zerocase");
                    0
                },
                Some(when) => {
                    // println!("Last spoken at {}. Current turn is {}, age is {}", when, turn, turn - when);
                    turn - when - 1
                },
            };

            // Now that we've looked up the previous, we can actually update the map
            // with what happened on the last turn
            last_spoken.insert(previous, turn - 1);

            if turn < 20 {
                println!("On turn {}. This number is {}", turn, this_num);
            }

            previous = this_num;
        }

        previous.to_string()
    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        todo!()
    }
}

impl AocBuilder for Day15 {
    fn new(input: &String) -> Result<Self, &str> {

        Ok(Self {
            starting_numbers: input.split(",").map(|n| usize::from_str_radix(n, 10).expect("input has valid numbers")).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_one() {
        assert_eq!(Day15::new(&"0,3,6".to_string()).expect("input is valid").solve_part_1(), "436");
    }

    #[test]
    fn day15_two() {
        assert_eq!(Day15::new(&"1,3,2".to_string()).expect("input is valid").solve_part_1(), "1");
    }

    #[test]
    fn day15_three() {
        assert_eq!(Day15::new(&"2,1,3".to_string()).expect("input is valid").solve_part_1(), "10");
    }

    #[test]
    fn day15_four() {
        assert_eq!(Day15::new(&"1,2,3".to_string()).expect("input is valid").solve_part_1(), "27");
    }

    #[test]
    fn day15_five() {
        assert_eq!(Day15::new(&"2,3,1".to_string()).expect("input is valid").solve_part_1(), "78");
    }

    #[test]
    fn day15_six() {
        assert_eq!(Day15::new(&"3,2,1".to_string()).expect("input is valid").solve_part_1(), "438");
    }

    #[test]
    fn day15_seven() {
        assert_eq!(Day15::new(&"3,1,2".to_string()).expect("input is valid").solve_part_1(), "1836");
    }
}
