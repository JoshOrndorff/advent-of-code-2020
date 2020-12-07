// 3397 is too low
// 3398 is also too low, although this one was just a guess anyway
// 5000 is too high. Also a guess just to (hopefully) confirm that it is lower than part 1 (which it must be)

use crate::{Aoc, AocBuilder};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Day6 {
    /// Tripply nested data. The three layers of nesting (from outer to inner) are
    /// Groups on the plane
    /// Members of the group
    /// Answers given by a group member
    groups_answers: Vec<Vec<HashSet<char>>>,
}

impl Aoc for Day6 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        // self.groups_answers
        //     .iter()
        //     .map(|group| group.iter().fold_first(|a, b| a.union(b).collect()).unwrap().len())
        //     .sum()
        //     .to_string()
        let mut sum_of_all_groups = 0;
        for group in &self.groups_answers {
            let mut all_answers : HashSet<char> = HashSet::new();
            for answers in group {
                // WTF, why can't I do the union of these sets!?
                //all_answers = answers.union(&all_answers).collect();
                for answer in answers {
                    all_answers.insert(*answer);
                }
            }
            sum_of_all_groups += all_answers.len();
        }
        sum_of_all_groups.to_string()
    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        println!("Warning! Day 6 Part 2 does not solve my input, although it does solve the test case.");

        let mut sum_of_all_groups = 0;
        for group in &self.groups_answers {
            println!("About to analyze group {:?}. Sum so far is {}", group, sum_of_all_groups);
            let mut group_total = 0;
            for letter in "abcdefghijklmnopqrstuvwxyz".chars() {
                let mut everyone_has_this_letter = true;
                for answers in group {
                    everyone_has_this_letter = everyone_has_this_letter && answers.contains(&letter);
                }
                if everyone_has_this_letter {
                    println!("Everyone has letter {}", letter);
                    group_total += 1;
                }
            }
            sum_of_all_groups += group_total;
        }
        sum_of_all_groups.to_string()
    }
}

impl AocBuilder for Day6 {
    fn new(input: &String) -> Result<Self, &str> {
        let groups_answers = input
            .split("\n\n")
            .map(|group_str| {
                group_str
                    .split("\n")
                    .map(|answers_str| answers_str.chars().collect())
                    .collect()
            })
            .collect();
        Ok(Self {
            groups_answers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_problem() -> Day6 {
        let example_input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        Day6::new(&example_input.to_string()).expect("Example input is valid")
    }

    #[test]
    fn part_one() {
        assert_eq!(get_test_problem().solve_part_1(), "11");
    }

    #[test]
    fn part_two() {
        assert_eq!(get_test_problem().solve_part_2(), "6");
    }
}
