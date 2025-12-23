# Advent of Code 2025 🎄

Rust-based solutions for [Advent of Code 2025](https://adventofcode.com/2025).

## 🚀 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70.0 or later)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

## 📁 Project Structure

This project uses a Cargo workspace to organize solutions for each day:

```
advent_of_code_2025/
├── Cargo.toml           # Workspace configuration
├── day01/               # Day 1 solution
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   └── input.txt
├── day02/               # Day 2 solution
│   └── ...
└── README.md
```

Each day's solution is a separate binary crate within the workspace, making it easy to:
- Run individual solutions independently
- Share common utilities across days
- Keep dependencies isolated when needed

## 🛠️ Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/gippy/advent_of_code_2025.git
   cd advent_of_code_2025
   ```

2. **Verify Rust installation**
   ```bash
   cargo --version
   rustc --version
   ```

## 📝 Creating a New Day Solution

To create a new day's solution (e.g., Day 1):

```bash
# Create a new binary crate
cargo new day01

# Add it to the workspace members in Cargo.toml
# members = ["day01"]
```

Or manually create the structure:
```bash
mkdir -p day01/src
touch day01/input.txt
```

Create `day01/Cargo.toml`:
```toml
[package]
name = "day01"
version.workspace = true
edition.workspace = true
authors.workspace = true

[dependencies]
```

Create `day01/src/main.rs`:
```rust
use std::fs;

fn main() {
    let input = fs::read_to_string("day01/input.txt")
        .expect("Failed to read input file");
    
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> usize {
    // Your solution here
    0
}

fn solve_part2(input: &str) -> usize {
    // Your solution here
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "example input";
        assert_eq!(solve_part1(input), 0);
    }

    #[test]
    fn test_part2_example() {
        let input = "example input";
        assert_eq!(solve_part2(input), 0);
    }
}
```

## 🏃 Running Solutions

```bash
# Run a specific day's solution
cargo run --bin day01

# Run with release optimizations (faster for complex solutions)
cargo run --release --bin day01

# Run tests for a specific day
cargo test --bin day01

# Run all tests in the workspace
cargo test

# Build all solutions
cargo build --release
```

## 🧪 Testing

Each day's solution should include unit tests with example inputs from the problem statement:

```bash
# Test a specific day
cargo test --bin day01

# Test with output
cargo test --bin day01 -- --nocapture

# Run tests in release mode (faster)
cargo test --release
```

## 📋 Development Workflow

1. **Read the day's problem** at https://adventofcode.com/2025
2. **Create a new day crate** using the template above
3. **Add example inputs** as test cases
4. **Implement part 1** and verify with tests
5. **Implement part 2** after solving part 1
6. **Optimize if needed** (use `cargo build --release`)

## ⚙️ Editor Setup (Zed)

This project is configured for use with [Zed editor](https://zed.dev/). The `.gitignore` is set up to exclude Zed-specific files (`.zed/`, `.zed.toml`).

For the best experience with Zed:
- Rust language server (rust-analyzer) should work out of the box
- Use `Cmd+Shift+P` (or `Ctrl+Shift+P`) to access the command palette
- Run tasks directly from the editor

## 🤝 Contributing

Feel free to explore different approaches and optimizations! Some ideas:
- Add benchmarks for performance comparison
- Create shared utility functions in a common crate
- Experiment with different algorithms

## 📚 Resources

- [Advent of Code 2025](https://adventofcode.com/2025)
- [Rust Documentation](https://doc.rust-lang.org/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rust Book](https://doc.rust-lang.org/book/)

## 📄 License

See [LICENSE](LICENSE) file for details.

---

Happy coding! 🦀✨
