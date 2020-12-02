use crate::{Aoc, AocBuilder};

#[derive(Debug)]
pub struct Policy {
    min: u8,
    max: u8,
    letter: char,
}

type Pair = (Policy, String);

pub struct Day2 {
    passwords: Vec<Pair>
}

impl Aoc for Day2 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        self.passwords
            .iter()
            .map(|p| if passes_sled_rules(p) {1} else {0})
            .sum::<u32>()
            .to_string()
    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        self.passwords
            .iter()
            .map(|p| if passes_toboggan_rules(p) {1} else {0})
            .sum::<u32>()
            .to_string()
    }
}

impl AocBuilder for Day2 {
    fn new(input: &String) -> Result<Self, &str> {
        let mut passwords = Vec::new();
        for l in input.lines() {
            passwords.push(parse(l)?);
        }
        Ok(Self {passwords})
    }
}

/// Parses one line of string input into useable data
/// TODO redo this with nom
fn parse(line: &str) -> Result<Pair, &str> {
    let colon_index = line.find(":").ok_or("each line contains exactly one colon")?;
    let dash_index = line.find("-").ok_or("each line contains exactly one dash")?;

    Ok((
        Policy {
            min: u8::from_str_radix(&line[..dash_index], 10)
                .map_err(|_e| "couldn't parse min occurences")?,
            max: u8::from_str_radix(&line[dash_index + 1 .. colon_index - 2], 10)
                .map_err(|_e| "couldn't parse max occurences")?,
            letter: line[colon_index - 1 .. colon_index].chars().next().expect("letter is a valid 1-char str"),
        },
        line[colon_index + 2 ..].to_string()
    ))
}

fn passes_sled_rules(pair: &Pair) -> bool {
    let (policy, password) = pair;
    let occurences = password.chars().filter(|c| c == &policy.letter).count();
    (policy.min as usize) <= occurences && occurences <= (policy.max as usize)
}

fn passes_toboggan_rules(pair: &Pair) -> bool {
    let (policy, password) = pair;

    // Grab characters. Indices are off by one because problem is worded in 1-based indexing
    let first = password.chars().nth((policy.min - 1).into()).expect("First character index is valid");
    let second = password.chars().nth((policy.max - 1).into()).expect("Second character index is valid");

    (first == policy.letter || second == policy.letter) && first != second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sled_one () {
        let pair = parse("1-3 a: abcde").expect("test case input parses");
        assert!(passes_sled_rules(&pair));
    }

    #[test]
    fn sled_two () {
        let pair = parse("1-3 b: cdefg").expect("test case input parses");
        assert!(!passes_sled_rules(&pair));
    }

    #[test]
    fn sled_three () {
        let pair = parse("2-9 c: ccccccccc").expect("test case input parses");
        assert!(passes_sled_rules(&pair));
    }

    #[test]
    fn toboggan_one () {
        let pair = parse("1-3 a: abcde").expect("test case input parses");
        assert!(passes_toboggan_rules(&pair));
    }

    #[test]
    fn toboggan_two () {
        let pair = parse("1-3 b: cdefg").expect("test case input parses");
        assert!(!passes_toboggan_rules(&pair));
    }

    #[test]
    fn toboggan_three () {
        let pair = parse("2-9 c: ccccccccc").expect("test case input parses");
        assert!(!passes_toboggan_rules(&pair));
    }
}
