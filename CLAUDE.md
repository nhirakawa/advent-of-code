# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Run Commands

- **Build**: `cargo build --release` or `just build`
- **Run latest solution**: `just latest` (runs latest day with debug logging)
- **Run all solutions**: `just all` or `./target/release/advent-of-code all`
- **Run specific year**: `./target/release/advent-of-code 2024 all`
- **Run specific day**: `./target/release/advent-of-code 2024 1`
- **Unit tests**: `cargo nextest run` (one year: `cargo nextest run -p year_2024`; tests named `*_slow` are skipped; include them with `--ignore-default-filter`)
- **Test mode**: `./target/release/advent-of-code --test all` (compares against expected outputs in `output/` directory)

## Project Architecture

This is a Rust-based Advent of Code solver with solutions spanning 2015-2024. The project uses a macro-based architecture to minimize boilerplate across years and days.

### Key Components

- **Cargo workspace**: The root package is the `advent-of-code` binary (`src/main.rs`); each year is its own library crate under `crates/`, plus a shared `common` crate. Editing one year only recompiles that crate and relinks the binary
- **`advent_year!` macro**: Generates boilerplate for each year crate, automatically creating the `solution()` function that dispatches to individual day modules
- **Year crates** (`crates/year_XXXX/src/lib.rs`): Simple files that invoke the macro, e.g., `common::advent_year!(2024);`
- **Day modules** (`crates/year_XXXX/src/day_*.rs`): Individual solution files with `part_one()` and `part_two()` functions
- **Common utilities** (`crates/common/`): Shared parsing, math, and debugging utilities, imported as `common::...`

### Directory Structure

- `input/year-XXXX/day-X.txt`: Problem inputs
- `output/year-XXXX/day-X/part-X.txt`: Expected outputs for testing
- `debug/`: Generated debug files (graphs, visualizations)
- `test_input/`: Small test inputs for development
- `crates/year_XXXX/`: Solution crate for each year
- `crates/common/`: Shared utilities and the `advent_year!` macros

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
2. The macro handles day dispatching automatically; a new year also needs entries in the root `Cargo.toml`, `run_day` in `src/main.rs`, and the `Year` enum
3. Add any extra dependencies to the year crate's `Cargo.toml` as `dep.workspace = true` (versions live in the root `[workspace.dependencies]`)
4. Day 25 only has part_one (no part_two function needed)

### Dependencies and Utilities

- **nom**: Primary parsing library (preferred over regex when possible)
- **itertools**: Iterator extensions
- **rayon**: Parallel processing
- **Common utilities**: `crates/common/src/parse.rs` for nom parsers, `crates/common/src/debug.rs` for visualization

### Special Cases

- **2019**: Has an additional `computer` module for IntCode virtual machine
- **Debug output**: Some solutions generate visualizations in `debug/` directory
- **Performance target**: Solutions should run under 1 second, ideally under 100ms
