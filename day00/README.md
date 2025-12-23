# Day 00 - Template

This is a template/example day to demonstrate the project structure.

## Structure

- `src/main.rs` - Main solution code with `solve_part1` and `solve_part2` functions
- `input.txt` - Puzzle input (this is just example data)
- `Cargo.toml` - Package configuration

## Usage

```bash
# Run the solution
cargo run --bin day00

# Run tests
cargo test --bin day00

# Run in release mode (optimized)
cargo run --release --bin day00
```

## Creating a New Day

To create a new day (e.g., Day 1), you can:

1. Copy this directory: `cp -r day00 day01`
2. Update `day01/Cargo.toml` to change the name to "day01"
3. Update references in `day01/src/main.rs` to read from "day01/input.txt"
4. Add your puzzle input to `day01/input.txt`

Or use cargo:

```bash
cargo new day01 --name day01
# Then copy the template structure from day00
```
