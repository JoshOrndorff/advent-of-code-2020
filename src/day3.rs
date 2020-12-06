use crate::{Aoc, AocBuilder};

pub struct Day3 {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Aoc for Day3 {
    /// Get the solution to part 1
    fn solve_part_1(&self) -> String {
        self.trees_on_slope(3, 1).to_string()
    }
    /// Get the solution to part 2
    fn solve_part_2(&self) -> String {
        (self.trees_on_slope(1, 1)
            * self.trees_on_slope(3, 1)
            * self.trees_on_slope(5, 1)
            * self.trees_on_slope(7, 1)
            * self.trees_on_slope(1, 2))
        .to_string()
    }
}

impl AocBuilder for Day3 {
    fn new(input: &String) -> Result<Self, &str> {
        Ok(Self {
            grid: input.lines().map(|l| l.chars().collect()).collect(),
            height: input.lines().count(),
            width: input
                .lines()
                .next()
                .expect("input contains at least one row")
                .chars()
                .count(),
        })
    }
}

impl Day3 {
    fn trees_on_slope(&self, run: usize, drop: usize) -> usize {
        let mut row = 0;
        let mut col = 0;
        let mut trees = 0;

        while row < self.height {
            let current_location = self
                .grid
                .get(row)
                .expect("row index is in range")
                .get(col)
                .expect("col index is in range");

            if current_location == &'#' {
                trees += 1;
            }

            row += drop;
            col = (col + run) % self.width;
        }

        trees
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_problem() -> Day3 {
        Day3::new(
            &"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
                .to_string(),
        )
        .expect("Test input is valid")
    }

    #[test]
    fn part_one() {
        let solution = get_test_problem().solve_part_1();
        assert_eq!(solution, "7");
    }

    #[test]
    fn part_two() {
        let solution = get_test_problem().solve_part_2();
        assert_eq!(solution, "336");
    }
}
