use crate::{Aoc, AocBuilder};
use std::collections::HashMap;

type Passport = HashMap<String, String>;

pub struct Day4 {
    passports: Vec<Passport>,
}

impl Aoc for Day4 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        let mut valid_count = 0;
        for passport in &self.passports {
            if is_valid(passport) {
                valid_count += 1;
            }
        }

        valid_count.to_string()
    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        todo!()
    }
}

impl AocBuilder for Day4 {
    fn new(input: &String) -> Result<Self, &str> {

        Ok(Self {
            passports: input.replace("\n", " ").trim().split("  ").map(|s| parse(s)).collect(),
        })
    }
}

/// Parses one line of string input into useable data
/// TODO redo this with nom
fn parse(line: &str) -> Passport {
    let pairs = line.split(" ");

    let mut passport = HashMap::new();

    for pair in pairs {
        let colon_index = pair.find(":").expect("each pair has exactly one colon");
        let key = pair[..colon_index].to_string();
        let value = pair[colon_index + 1..].to_string();

        passport.insert(key, value);
    }

    passport
}

fn is_valid(passport: &Passport) -> bool {
    let required_fields = vec![
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
        //"cid",
    ];

    required_fields.iter().map(|field| passport.contains_key(&field.to_string())).fold(true, |acc, next| acc && next)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert!(false);
    }
}
