# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Run Commands

- **Build**: `cargo build --release` or `just build`
- **Run latest solution**: `just latest` (runs latest day with debug logging)
- **Run all solutions**: `just all` or `./target/release/advent-of-code all`
- **Run specific year**: `./target/release/advent-of-code 2024 all`
- **Run specific day**: `./target/release/advent-of-code 2024 1`
- **Test mode**: `./target/release/advent-of-code --test all` (compares against expected outputs in `output/` directory)

## Project Architecture

This is a Rust-based Advent of Code solver with solutions spanning 2015-2024. The project uses a macro-based architecture to minimize boilerplate across years and days.

### Key Components

- **`advent_year!` macro**: Generates boilerplate for each year module, automatically creating the `solution()` function that dispatches to individual day modules
- **Year modules** (`year_XXXX.rs`): Simple files that invoke the macro, e.g., `crate::advent_year!(2024);`
- **Day modules** (`year_XXXX/day_*.rs`): Individual solution files with `part_one()` and `part_two()` functions
- **Common utilities** (`src/common/`): Shared parsing, math, and debugging utilities

### Directory Structure

- `input/year-XXXX/day-X.txt`: Problem inputs
- `output/year-XXXX/day-X/part-X.txt`: Expected outputs for testing
- `debug/`: Generated debug files (graphs, visualizations)
- `test_input/`: Small test inputs for development
- `src/year_XXXX/`: Solution modules for each year

### Day Implementation Pattern

Each day module implements:

```rust
pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    // Solution logic
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    // Solution logic
}
```

### Adding New Solutions

1. Use `python bootstrap.py` to generate stub files for new years/days
2. The macro handles all dispatching automatically
3. Day 25 only has part_one (no part_two function needed)

### Dependencies and Utilities

- **nom**: Primary parsing library (preferred over regex when possible)
- **itertools**: Iterator extensions
- **rayon**: Parallel processing
- **Common utilities**: `src/common/parse.rs` for nom parsers, `src/common/debug.rs` for visualization

### Special Cases

- **2019**: Has an additional `computer` module for IntCode virtual machine
- **Debug output**: Some solutions generate visualizations in `debug/` directory
- **Performance target**: Solutions should run under 1 second, ideally under 100ms
