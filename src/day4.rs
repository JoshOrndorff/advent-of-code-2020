use crate::{Aoc, AocBuilder};
use std::collections::HashMap;

type Passport = HashMap<String, String>;

pub struct Day4 {
    passports: Vec<Passport>,
}

impl Aoc for Day4 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        self.passports
            .iter()
            .filter(|p| has_required_fields(&p))
            .count()
            .to_string()
    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        self.passports
            .iter()
            .filter(|p| has_required_fields(&p))
            .filter(|p| has_valid_fields(&p))
            .count()
            .to_string()
    }
}

impl AocBuilder for Day4 {
    fn new(input: &String) -> Result<Self, &str> {
        Ok(Self {
            passports: input
                .replace("\n", " ")
                .trim()
                .split("  ")
                .map(|s| parse(s))
                .collect(),
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

/// Validates that a single passport has all the required fields
fn has_required_fields(passport: &Passport) -> bool {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    required_fields
        .iter()
        .map(|field| passport.contains_key(&field.to_string()))
        .fold(true, |acc, next| acc && next)
}

/// Validates that the fields of a single passport are individually valid.
/// Assumes that passport passed in _has_ all required fields.
fn has_valid_fields(passport: &Passport) -> bool {
    validate_byr(&passport.get("byr").unwrap())
        && validate_iyr(&passport.get("iyr").unwrap())
        && validate_eyr(&passport.get("eyr").unwrap())
        && validate_hgt(&passport.get("hgt").unwrap())
        && validate_hcl(&passport.get("hcl").unwrap())
        && validate_ecl(&passport.get("ecl").unwrap())
        && validate_pid(&passport.get("pid").unwrap())
}

/// Validates a birth year
// len is naive and counts the number of bytes. value.chars().count() might be better
fn validate_byr(value: &str) -> bool {
    // First check the length
    if value.len() != 4 {
        return false;
    };

    match u16::from_str_radix(value, 10) {
        Err(_) => false,
        Ok(year) => year >= 1920 && year <= 2002,
    }
}

fn validate_iyr(value: &str) -> bool {
    // First check the length
    if value.len() != 4 {
        return false;
    };

    match u16::from_str_radix(value, 10) {
        Err(_) => false,
        Ok(year) => year >= 2010 && year <= 2020,
    }
}

fn validate_eyr(value: &str) -> bool {
    // First check the length
    if value.len() != 4 {
        return false;
    };

    match u16::from_str_radix(value, 10) {
        Err(_) => false,
        Ok(year) => year >= 2020 && year <= 2030,
    }
}

/// Heigt units
#[derive(Debug)]
enum Units {
    Cm,
    In,
}

fn validate_hgt(value: &str) -> bool {
    // Figure out whether there is even a valid unit label
    let units = if value.find("in").is_some() {
        Units::In
    } else if value.find("cm").is_some() {
        Units::Cm
    } else {
        return false;
    };

    // Just using bytes again here. shrug
    let number = u16::from_str_radix(&value[..value.len() - 2], 10)
        .ok()
        .expect("If there is a valid label, the number should parse");

    println!("number is {}", number);
    // Now validate the number
    match units {
        Units::In => number >= 59 && number <= 76,
        Units::Cm => number >= 150 && number <= 193,
    }
}

fn validate_hcl(value: &str) -> bool {
    if value.chars().next() != Some('#') {
        return false;
    }

    u32::from_str_radix(&value[1..], 16).is_ok()
}

fn validate_ecl(value: &str) -> bool {
    value == "amb"
        || value == "blu"
        || value == "brn"
        || value == "gry"
        || value == "grn"
        || value == "hzl"
        || value == "oth"
}

fn validate_pid(value: &str) -> bool {
    if value.chars().count() != 9 {
        return false;
    }

    u32::from_str_radix(&value, 10).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_hgt_1() {
        assert!(validate_hgt(&"167cm"));
    }

    #[test]
    fn valid_fields_1() {
        assert!(!has_valid_fields(&parse(
            &"eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
        )));
    }

    #[test]
    fn valid_fields_2() {
        assert!(!has_valid_fields(&parse(
            &"iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946"
        )));
    }

    #[test]
    fn valid_fields_3() {
        assert!(!has_valid_fields(&parse(
            &"hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"
        )));
    }

    #[test]
    fn valid_fields_4() {
        assert!(!has_valid_fields(&parse(
            &"hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"
        )));
    }

    #[test]
    fn valid_fields_5() {
        assert!(has_valid_fields(&parse(
            &"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f"
        )));
    }

    #[test]
    fn valid_fields_6() {
        assert!(has_valid_fields(&parse(
            &"eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
        )));
    }

    #[test]
    fn valid_fields_7() {
        assert!(has_valid_fields(&parse(
            &"hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022"
        )));
    }

    #[test]
    fn valid_fields_8() {
        assert!(has_valid_fields(&parse(
            &"iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
        )));
    }

    #[test]
    fn valid_byr() {
        assert!(validate_byr(&"2002"));
    }

    #[test]
    fn invalid_byr() {
        assert!(!validate_byr(&"2003"));
    }

    #[test]
    fn valid_hgt_1() {
        assert!(validate_hgt(&"60in"));
    }

    #[test]
    fn valid_hgt_2() {
        assert!(validate_hgt(&"190cm"));
    }

    #[test]
    fn invalid_hgt_1() {
        assert!(!validate_hgt(&"190in"));
    }

    #[test]
    fn invalid_hgt_2() {
        assert!(!validate_hgt(&"190"));
    }
}
