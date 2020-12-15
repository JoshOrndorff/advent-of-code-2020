use std::collections::HashMap;

use crate::{Aoc, AocBuilder};


pub struct Day10 {
    adapters: Vec<usize>,
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
        how_many_ways(&self.adapters, 0, &mut HashMap::new()).to_string()
    }
}

/// Recursive function that determines how many different ways the given joltage adapters can be combined necessarily
/// using the last one. The results are memoized as they are computed so that the same calculations are not made again later.
fn how_many_ways(adapters: &[usize], current: usize, memoized: &mut HashMap<usize, usize>) -> usize {
    // println!("Current: {}, Target: {}, Adapters Left: {:?}", current, target, adapters);

    // First we check for an answer associated with the current joltage. If we know one, return it.
    if let Some(answer) = memoized.get(&current) {
        return *answer;
    }

    if adapters.len() == 0 {
        return 1;
    }

    // We know we can always use the very next adapter, so no need to check.
    let mut ways =  how_many_ways(&adapters[1..], adapters[0], memoized);

    // Sometimes we can skip to the second adapter
    if adapters.len() >= 2 && adapters[1] - current <= 3 {
        ways += how_many_ways(&adapters[2..], adapters[1], memoized);

        // Sometimes we can skip to the third. But only if skipping to the second worked.
        if adapters.len() >= 3 && adapters[2] - current <= 3 {
            ways += how_many_ways(&adapters[3..], adapters[2], memoized);

            // We can never skip to the third because each adapter is unique and a max three-jolt difference is allowed.
        }
    }

    // Update the memoization table and return the answer
    memoized.insert(current, ways);
    ways
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
    fn day10_ex1_part1() {
        assert_eq!(example_one().solve_part_1(), (7*5).to_string());
    }

    #[test]
    fn day10_ex1_part2() {
        assert_eq!(example_one().solve_part_2(), "8");
    }

    #[test]
    fn day10_ex2_part1() {
        assert_eq!(example_two().solve_part_1(), (22*10).to_string());
    }

    #[test]
    fn day10_ex2_part2() {
        assert_eq!(example_two().solve_part_2(), "19208");
    }
}
