# advent-of-code

Advent of Code solutions in Rust

## Goals

In order of priority:

1. Solve a variety of problems using Rust
1. Minimal usage of dependencies (ideally only nom for parsing input)
1. Both parts should run under 1 second (where possible)
   a. Ideally, both parts should run in under 100 ms
1. Write idiomatic code that is readable and maintainable/extensible
   a. Code should not be overly verbose, but it is a non-goal to minimize lines of code (i.e. no code golf)
1. Code should generally be testable, especially in later days

## Development

### Architecture

This project uses a macro-based architecture to minimize boilerplate:

- **`advent_year!` macro**: Automatically generates year modules and solution dispatching
- **Year modules** (`year_XXXX.rs`): Simple files that invoke the macro
- **Day modules** (`year_XXXX/day_*.rs`): Individual solutions with `part_one()` and `part_two()` functions
- **Common utilities** (`src/common/`): Shared parsing, math, and debugging tools

Use `python bootstrap.py` to generate stub files for new solutions.

### LLMs

I use LLMs for boilerplate tasks - restructuring input/output functions, adding traits, changing function signatures for solution functions - but I do not use LLMs for solving problems.

## Solved

- [ ] 2022
- [ ] 2021
- [x] 2020
- [ ] 2019
- [ ] 2018
- [ ] 2017
- [ ] 2016
- [ ] 2015
