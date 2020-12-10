use crate::{Aoc, AocBuilder};
use std::collections::{HashSet, HashMap};

pub struct Day7 {
    rules: HashMap<String, Vec<(usize, String)>>,
}

impl Aoc for Day7 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        //First build a reverse map.
        //TODO If this is useful in part two also, maybe add it to the struct
        let mut reverse_map = HashMap::new();
        for (outer_color, inner_pairs) in &self.rules {
            // Ensure the outermost bags appear in the reverse map.
            if reverse_map.get(outer_color).is_none() {
                reverse_map.insert(outer_color, Vec::new());
            }

            for (_, inner_color) in inner_pairs {

                // In case there isn't yet a vec setup
                if reverse_map.get(inner_color).is_none() {
                    reverse_map.insert(inner_color, Vec::new());
                }

                reverse_map.get_mut(inner_color).expect("just initialized it").push(outer_color);
            }
        }

        // Use the recursive algorithm to find the outer bags.
        let mut initial_open_set = HashSet::new();
        initial_open_set.insert("shiny gold".to_string());
        let num_outers = find_all_outers(initial_open_set, HashSet::new(), &reverse_map).iter().count() - 1;

        num_outers.to_string()
    }

    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        let total_count = self.count_inners(&"shiny gold".to_string());
        (total_count - 1).to_string()
    }

}

impl Day7 {
    /// Recursive helper method to count how many total bags exist in a valid tree of the given outer color.
    ///
    /// This naively recomputes subtrees when they are visited. It could be improved by memoizing that data,
    /// but it turned out that wasn't necessary for the input given.
    fn count_inners(&self, target: &String) -> usize {
        let pairs = self.rules.get(target).expect("target color is in ruleset");

        // Initialize to 1 for this bag.
        let mut total_inners = 1;
        for (count, inner) in pairs {
            println!("count here is {}", count);
            total_inners += count * self.count_inners(inner);
        }

        println!("Finished counting for {}, count is {}", target, total_inners);

        total_inners
    }
}

/// A recursive function that returns all bags that can surround the given open set, plus any bags that are
/// in the closed set. The intended usage is to pass an open set with a single item and an empty closed set.
fn find_all_outers(mut open: HashSet<String>, mut closed: HashSet<String>, reverse_map: &HashMap<&String, Vec<&String>>) -> HashSet<String> {
    // We terminate if the open set is empty. Otherwise, we grab an element to process.
    let current = match open.iter().next() {
        Some(e) => e.to_string(),
        None => return closed,
    };

    // Move the current element from the open set to the colsed set.
    closed.insert(open.take(&current).expect("current element is in open set, or iterator wouldn't have supplied it."));

    // Add the new round of outers to the closed set.
    let new_outers = reverse_map.get(&current).expect("Current element exists in reverse map.");
    for new_outer in new_outers {
        open.insert(new_outer.to_string());
    }

    // Recurse
    find_all_outers(open, closed, reverse_map)
}

impl AocBuilder for Day7 {
    fn new(input: &String) -> Result<Self, &str> {
        let mut rules = HashMap::new();
        for line in input.lines() {
            let (outer_color, contents) = parse_rule(line);
            rules.insert(outer_color, contents);
        }

        Ok(Self { rules })
    }
}

/// Parse one line of input into outer color an contents. Input is of the form:
/// muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
/// The entire thing assumes the strings are valid ascii (one byte per char)
fn parse_rule(input: &str) -> (String, Vec<(usize, String)>) {
    let split_index = input.find(" bags contain ").expect("Input line is formatted correctly");
    let outer_color = &input[..split_index];
    let contents_str = &input[split_index + 14 .. input.len() - 1];

    (outer_color.to_string(), parse_pairs(contents_str))
}

fn parse_pairs(input: &str) -> Vec<(usize, String)> {
    if input == "no other bags" {
        return Vec::new()
    }

    input.split(", ").map(parse_pair).collect()
}

/// Parses a single (quantity, color) pair of the forms:
/// 9 faded blue bags.
/// 3 muted yellow bags
/// 1 dark orange bag
/// 1 light blue bag.
fn parse_pair(input: &str) -> (usize, String) {
    let bag_index = input.find(" bag").unwrap();
    let trimmed = &input[..bag_index];
    let space_index = trimmed.find(" ").unwrap();
    let number_str = &trimmed[..space_index];
    let number = usize::from_str_radix(number_str, 10).unwrap();
    let color = &trimmed[space_index + 1..];
    (number, color.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_problem_1() -> Day7 {
        let example_input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        Day7::new(&example_input.to_string()).expect("Example input is valid")
    }

    #[test]
    fn part_one() {
        assert_eq!(test_problem_1().solve_part_1(), "4");
    }

    #[test]
    fn first_part_two() {
        assert_eq!(test_problem_1().solve_part_2(), "32");
    }
}
