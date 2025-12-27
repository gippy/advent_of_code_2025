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
├── ...
├── day12/               # Day 12 solution
└── README.md
```

Each day's solution is a separate binary crate within the workspace.

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
2. **Paste your input** into `dayXX/input.txt`
3. **Implement the solution** in `dayXX/src/main.rs`
4. **Run and verify** using `cargo run --bin dayXX`

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
