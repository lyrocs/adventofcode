# Advent of Code - Rust Solutions

My solutions to [Advent of Code](https://adventofcode.com/) puzzles written in Rust.

## Project Structure

```
adventofcode/
├── src/
│   ├── bin/
│   │   ├── day01.rs    # Day 1 solution
│   │   ├── day02.rs    # Day 2 solution
│   │   └── ...
│   └── lib.rs          # Shared utilities
├── inputs/
│   ├── day01.txt       # Day 1 input
│   ├── day02.txt       # Day 2 input
│   └── ...
└── Cargo.toml
```

## Usage

### Run a specific day

```bash
cargo run --bin day01
```

### Run with release optimizations

```bash
cargo run --release --bin day01
```

### Run tests

```bash
cargo test
```

## Adding a New Day

1. Create input file: `inputs/dayXX.txt`
2. Create solution: `src/bin/dayXX.rs`
3. Run: `cargo run --bin dayXX`
