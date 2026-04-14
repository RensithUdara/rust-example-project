# Chapter 1: Basics - Hello Rust! 🦀

## What is Rust?

Rust is a **systems programming language** that provides:
- ✅ **Memory Safety** without garbage collection
- ✅ **Speed** comparable to C/C++
- ✅ **Concurrency** made safe and easy
- ✅ **Expressive** syntax for complex ideas

## Key Features

| Feature | Benefit |
|---------|---------|
| Ownership System | Prevents memory leaks at compile time |
| No Garbage Collector | Predictable performance |
| Type System | Catches bugs before runtime |
| Pattern Matching | Powerful control flow |
| Zero-Cost Abstractions | High-level code runs as fast as low-level |

## Installation

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

## Creating Your First Project

```bash
# Create a new Rust project
cargo new hello_rust

# Navigate to the project
cd hello_rust

# Run the project
cargo run
```

## Project Structure

```
hello_rust/
├── Cargo.toml       # Project configuration & dependencies
├── src/
│   └── main.rs      # Entry point of your program
└── target/          # Compiled binaries
```

## Your First Program

```rust
fn main() {
    println!("Hello, World!");
}
```

Breaking it down:
- `fn main()` - The main function where your program starts
- `println!` - A macro (notice the `!`) that prints text
- `;` - Rust uses semicolons to end statements

## Macros vs Functions

In Rust, you'll see `!` which indicates a **macro** (more powerful than functions):

```rust
println!()   // Macro - can take variable number of args
print!()     // Another macro
dbg!()       // Debugging macro
```

## Common Commands

```bash
# Create a new project
cargo new project_name

# Build the project
cargo build

# Build for release (optimized)
cargo build --release

# Run the project
cargo run

# Run without compiling if already compiled
cargo run --release

# Check if code compiles without full build
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Lint code (find common mistakes)
cargo clippy
```

## The Rust Compiler

Rust has a very helpful compiler that catches errors **before runtime**:

```rust
fn main() {
    let x = 5;
    x = 10;  // ❌ ERROR: Cannot reassign immutable variable!
}
```

The compiler prevents you from making mistakes! This is a feature, not a limitation.

## Memory Safety Without Garbage Collection

```
    Rust Approach:
    ┌──────────────────────────────────────┐
    │ Compile Time Check (No Runtime Cost) │
    │ Ownership System Ensures Safety      │
    └──────────────────────────────────────┘
              ⬇
        Fast Code ✅

    Garbage Collector Approach:
    ┌──────────────────────────────────────┐
    │ Runtime Check (Performance Cost)     │
    │ GC Marks & Sweeps                    │
    └──────────────────────────────────────┘
              ⬇
        Slower (But easier) ❌
```

## Next Steps

You now understand:
- ✅ What Rust is and why it exists
- ✅ How to create and run a Rust project
- ✅ Basic syntax of a Rust program
- ✅ Common Cargo commands

**Ready to move forward?** Let's dive into variables and data types!
