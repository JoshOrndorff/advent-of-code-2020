use crate::{AOC, AOCBuilder};

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

impl AOC for Day2 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        let valid_count : u32 = self.passwords.iter().fold(0, |prev, pair| prev + if is_valid(pair) {1} else {0});
        valid_count.to_string()
    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        todo!()
    }
}

impl AOCBuilder for Day2 {
    fn new(input: &String) -> Result<Box<dyn AOC>, &str> {
        let mut passwords = Vec::new();
        for l in input.lines() {
            passwords.push(parse(l)?);
        }
        for (policy, password) in &passwords {
            println!("{:?}, {}", policy, password);
        }
        Ok(Box::new(Self {passwords}) as Box<dyn AOC>)
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

fn is_valid(pair: &Pair) -> bool {
    let (policy, password) = pair;
    let occurences = password.chars().filter(|c| c == &policy.letter).count();
    println!("Number of occurences: {}", occurences);
    (policy.min as usize) <= occurences && occurences <= (policy.max as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one () {
        let pair = parse("1-3 a: abcde").expect("test case input parses");
        assert!(is_valid(&pair));
    }

    #[test]
    fn test_two () {
        let pair = parse("1-3 b: cdefg").expect("test case input parses");
        assert!(!is_valid(&pair));
    }#[test]
    fn test_three () {
        let pair = parse("2-9 c: ccccccccc").expect("test case input parses");
        assert!(is_valid(&pair));
    }
}
