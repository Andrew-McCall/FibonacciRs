# FibonacciRs

FibonacciRs benchmarks Fibonacci number generators implemented in Rust and Python, highlighting performance differences and implementation styles.

## Features

* Recursive and iterative Fibonacci algorithms in Rust and Python.
* Execution speed benchmarking.
* Simple, minimal setup for easy extension and modification.

## Repository Structure

```
.
├── fib.py           # Python implementation
└── fib.rs           # Rust implementation
```

## Requirements

* Python 3.x
* Rust (via rustup)

## Usage

### Run Python Version

Run the following command in your shell:

```
python fib.py
```

### Run Rust Version

Run the following command in your shell:

```
cargo run --release
```

## Output

Both versions compute Fibonacci values for a predefined `n` (adjustable in source) and report execution time.

## License

MIT License Copyright (c) 2025 Andrew McCall
