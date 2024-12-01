# Advent of Code 2024 - Rust Solutions
This repository contains my solutions for Advent of Code 2024 implemented in Rust.

## Project Structure

```
advent-2024/
├── Cargo.toml
├── src/
│   ├── main.rs      # Main runner
│   ├── lib.rs       # Solution modules
│   ├── day01.rs     # Individual day solutions
│   └── day02.rs
├── inputs/          # Puzzle inputs
│   ├── day01.txt
│   └── day02.txt
└── tests/           # Additional tests
    ├── day01_test.rs
    └── day02_test.rs
```

## Running Solutions

Run a specific day's solution:

```bash
cargo run -- 1    # Runs day 1
cargo run -- 2    # Runs day 2
```

Run with automatic recompilation using cargo-watch:

```bash 
cargo watch -x "run -- 1"    # Rerun day 1 on any code change
```

## Dependencies

Key crates used in this project:

* itertools: Iterator combinators for elegant data manipulation
* regex: Regular expression support

Optional but recommended:

* num: Number theory operations
* petgraph: Graph algorithms
* pathfinding: Path finding algorithms

## Development Setup

Install Rust and Cargo:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install helpful tools:

```bash
cargo install cargo-watch    # Auto-recompilation
cargo install cargo-edit     # Dependency management
```

Clone and build:

```bash
git clone [your-repo-url]
cd advent-2024
cargo build
```

## Daily Solution Template

Each day's solution follows this pattern:

```rust
pub fn part1(input: &str) -> u32 {
    // Solution for part 1
    0
}

pub fn part2(input: &str) -> u32 {
    // Solution for part 2
    0
}
```

## Contributing

Feel free to open issues or PRs if you have suggestions for improvements!

## License

This project is licensed under the MIT License - see the LICENSE file for details.

