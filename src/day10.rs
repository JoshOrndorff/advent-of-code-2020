use std::collections::HashMap;

use crate::{Aoc, AocBuilder};


pub struct Day10 {
    adapters: Vec<usize>,
    //TODO should I calculate my devices input joltage?
}

impl Aoc for Day10 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        // Setup a map to keep track of the differences.
        let mut dist = HashMap::new();

        let mut previous = 0;
        for adapter in &self.adapters {
            let diff = adapter - previous;
            dist.insert(diff, dist.get(&diff).unwrap_or(&0) + 1);

            previous = *adapter;
        }

        // How many of the differences asked about exist. One extra for the 3-jolt because of the built-in adapter.
        let one_jolt_diff = dist.get(&1).unwrap_or(&0);
        let three_jolt_diff = dist.get(&3).unwrap_or(&0) + 1;

        (one_jolt_diff * three_jolt_diff).to_string()
    }

    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        todo!()
    }
}

impl AocBuilder for Day10 {
    fn new(input: &String) -> Result<Self, &str> {
        let mut adapters : Vec<usize> = input.trim().lines().map(|s| usize::from_str_radix(s, 10).expect("input contains valid numbers")).collect();
        adapters.sort();

        Ok(Self {
            adapters,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_one() -> Day10 {
        Day10::new(&"16
10
15
5
1
11
7
19
6
12
4
".to_string()).expect("example input is valid")
    }

    fn example_two() -> Day10 {
        Day10::new(&"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
".to_string()).expect("example input is valid")
    }

    #[test]
    fn test_one() {
        assert_eq!(example_one().solve_part_1(), (7*5).to_string());
    }

    #[test]
    fn test_two() {
        assert_eq!(example_two().solve_part_1(), (22*10).to_string());
    }
}
