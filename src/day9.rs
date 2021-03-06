use crate::{Aoc, AocBuilder};


pub struct Day9 {
    /// How big of a preamble we're considering
    preamble: usize,
    /// The sequence of numbers we're analyzing
    sequence: Vec<usize>,
}

impl Aoc for Day9 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        let mut start = 0;
        loop {
            let end = start + self.preamble + 1;
            let current_slice = &self.sequence[start..end];
            if !check_slice(current_slice) {
                return self.sequence[end - 1].to_string();
            }
            start += 1;
        }
    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        // This could be improved by remembering the solution from when we calculated part 1,
        // but it's not so bad this way either.
        let target = usize::from_str_radix(&self.solve_part_1(), 10).expect("part one has a numeric solution");

        // This is O(n^3) because of the summing. It is the first one that
        // has taken a non-neglegible amount of time.
        for i in 0..self.sequence.len() {
            for j in i + 1 .. self.sequence.len() {
                let contiguous = &self.sequence[i..j];
                if contiguous.iter().sum::<usize>() == target {
                    let min = contiguous.iter().min().expect("There was a min");
                    let max = contiguous.iter().max().expect("There was a max");
                    return (min + max).to_string();
                }
            }
        }
        panic!("Completed loop without finding colution");
    }
}

/// Checks the validity of the given slice. The preable is assumed to be the entirety
/// of the provided slice except the last character.
fn check_slice(s: &[usize]) -> bool {
    let preamble = &s[..s.len()];
    let check = s[s.len()-1];

    // Check pairwise
    for i in 0..preamble.len()-1 {
        for j in i+1..preamble.len()-1 {
            if preamble[i] + preamble[j] == check {
                return true;
            }
        }
    }

    // We didn't find any valid pairs
    false
}

impl AocBuilder for Day9 {
    fn new(input: &String) -> Result<Self, &str> {

        Ok(Self {
            preamble: 25,
            sequence: input.trim().lines().map(|s| usize::from_str_radix(s, 10).unwrap()).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_problem() -> Day9 {
        let mut p = Day9::new(&"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
".to_string()).expect("example input is valid");
        p.preamble = 5;
        p
    }

    #[test]
    fn day9_one() {
        assert_eq!(example_problem().solve_part_1(), "127");
    }

    #[test]
    fn day9_two() {
        assert_eq!(example_problem().solve_part_2(), "62");
    }

    #[test]
    fn check_slice_one() {
        assert!(check_slice(&[1,2,3,4,5,6,7,8,9]));
    }
}
