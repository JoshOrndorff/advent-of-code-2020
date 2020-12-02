# Advent of Code 2020

My solutions to https://adventofcode.com 2020.

## Using the CLI

```bash
# Compile
cargo build # builds to target/debug/advent-of-code-2020

# Solve day 10 problem with input on the CLI
advent-of-code-2020 --day 10 --input 1234

# Solve day 1 problem with input from a file
advent-of-code-2020 --day 10 --input-file inputs/day1.json

# Learn more
advent-of-code-2020 --help
```

## Lessons Learned

I'm trying to experiment with new techniques to improve my Rust chops.

* Using structopt to take CLI arguments
* Dynamically dispatching to solvers for specific days

Aspirational:
* Use wasm bindgen and make a website
