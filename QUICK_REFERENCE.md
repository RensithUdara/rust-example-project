# Rust Quick Reference Guide 📋

## Data Types Cheat Sheet

```rust
// Integers
let i: i32 = 42;        // Signed 32-bit
let u: u32 = 42;        // Unsigned 32-bit

// Floats
let f: f64 = 3.14;      // Double precision (default)
let f: f32 = 3.14;      // Single precision

// Boolean
let b: bool = true;

// Character
let c: char = 'A';

// String
let s: &str = "hello";              // String slice
let s: String = String::from("hi"); // Owned string

// Collections
let v: Vec<i32> = vec![1, 2, 3];    // Vector
let arr: [i32; 3] = [1, 2, 3];      // Array
let tup: (i32, &str) = (1, "hi");   // Tuple
```

## Ownership Quick Reference

```rust
// Stack (Copy)
let x = 5;
let y = x;          // Both valid (copied)

// Heap (Move)
let s1 = String::from("hello");
let s2 = s1;        // s1 invalid (moved)

// Borrowing
let s1 = String::from("hello");
let s2 = &s1;       // s1 still valid (borrowed)
let s3 = &mut s1;   // Mutable borrow (limited)
```

## Functions

```rust
fn name(param: Type) -> ReturnType {
    // expression (no semicolon)
    param + 1
}

// Or with statement
fn name(param: i32) -> i32 {
    let x = param + 1;
    x  // Return via expression
}
```

## Control Flow

```rust
// If-else
if x > 5 {
    println!("big");
} else if x > 2 {
    println!("medium");
} else {
    println!("small");
}

// As expression
let num = if x > 5 { 1 } else { 0 };

// Loop
loop {
    // infinite loop
    break;  // Exit
}

// While
while x < 10 {
    x += 1;
}

// For
for i in 1..=5 {
    println!("{}", i);
}

// For with iterator
for item in &vector {
    println!("{}", item);
}
```

## Pattern Matching

```rust
// Match
match value {
    1 => println!("one"),
    2 => println!("two"),
    3..=5 => println!("three to five"),
    _ => println!("other"),
}

// If let (single pattern)
if let Some(x) = option {
    println!("{}", x);
}

// While let
while let Some(x) = iterator.next() {
    println!("{}", x);
}
```

## Structs and Enums

```rust
// Struct
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 0 };

// Methods
impl Point {
    fn distance(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

// Enum
enum Status {
    Active,
    Inactive,
    Pending,
}

let s = Status::Active;
```

## Option and Result

```rust
// Option
let opt = Some(5);
match opt {
    Some(x) => println!("{}", x),
    None => println!("none"),
}

// Result
let res: Result<i32, String> = Ok(42);
match res {
    Ok(x) => println!("{}", x),
    Err(e) => println!("error: {}", e),
}

// Methods
opt.unwrap_or(0);
opt.map(|x| x * 2);
opt.and_then(|x| Some(x * 2));

// Question mark
fn process() -> Result<i32, String> {
    let val = divide(10, 2)?;  // Return if error
    Ok(val)
}
```

## Collections

```rust
// Vector
let mut v = vec![1, 2, 3];
v.push(4);
v.pop();
v[0];           // Access
v.get(0);       // Safe access

// HashMap
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", "value");
map.get("key");

// HashSet
use std::collections::HashSet;
let mut set = HashSet::new();
set.insert("value");
set.contains("value");
```

## Iterators

```rust
// Creating iterators
for item in &vec {}           // iter() - borrow
for item in &mut vec {}       // iter_mut() - mutable
for item in vec {}            // into_iter() - consume

// Adapters (lazy)
vec.iter()
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .take(3)

// Consumers (execute)
.collect::<Vec<_>>()
.sum::<i32>()
.count()
.find(|x| x == &5)
.any(|x| x % 2 == 0)
```

## Closures

```rust
let add_one = |x| x + 1;
let multiply = |x: i32| -> i32 { x * 2 };

// Capturing
let y = 5;
let add_y = |x| x + y;

// move
let s = String::from("hello");
let take = move || println!("{}", s);

// In iterators
vec.map(|x| x * 2)
```

## Error Handling

```rust
// Panic
panic!("Something went wrong!");

// Result handling
match divide(10, 2) {
    Ok(r) => println!("{}", r),
    Err(e) => println!("error: {}", e),
}

// Unwrap
let x = result.unwrap();

// Expect
let x = result.expect("division failed");

// Unwrap_or
let x = result.unwrap_or(0);

// Question mark
fn process() -> Result<i32, String> {
    let val = divide(10, 2)?;
    Ok(val)
}
```

## Traits and Generics

```rust
// Generic function
fn largest<T: PartialOrd>(items: &[T]) -> &T {
    &items[0]
}

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

// Trait
trait Animal {
    fn sound(&self) -> &str;
}

impl Animal for Dog {
    fn sound(&self) -> &str {
        "Woof!"
    }
}

// Trait bounds
fn print<T: Display>(x: T) {
    println!("{}", x);
}

// Multiple bounds
fn process<T: Clone + Display>(x: T) {}

// Trait objects
fn get_animal() -> Box<dyn Animal> {
    Box::new(Dog)
}
```

## Useful Macros

```rust
println!("Hello");           // Print with newline
print!("Hello");             // Print without newline
dbg!(x);                     // Debug print variable
panic!("error");             // Panic with message
assert!(x > 0);              // Assert condition
assert_eq!(a, b);            // Assert equality
vec![1, 2, 3];               // Create vector
format!("{}", x);            // Format string
todo!();                     // Mark unimplemented
unreachable!();              // Mark unreachable code
```

## Common Methods

### String
```rust
let s = "hello";
s.len();                 // Length in bytes
s.chars().count();       // Character count
s.contains("ell");
s.starts_with("he");
s.trim();
s.to_uppercase();
s.split(" ");
```

### Vec
```rust
let v = vec![1, 2, 3];
v.len();
v.is_empty();
v.push(4);
v.pop();
v.remove(0);
v.contains(&1);
v.iter();
v.reverse();
v.sort();
```

### Iterator
```rust
lst.iter()
    .map(|x| x * 2)
    .filter(|x| x % 2 == 0)
    .take(5)
    .skip(2)
    .sum()
    .collect()
    .find()
    .any()
    .all()
    .count()
```

## Common Traits to Implement

```rust
#[derive(Debug)]           // Enable {:?} printing
#[derive(Clone)]           // Enable .clone()
#[derive(Copy)]            // Auto-copy semantics
#[derive(PartialEq)]       // Enable ==
#[derive(Default)]         // Enable Default::default()
```

## Common Errors & Fixes

| Error | Fix |
|-------|-----|
| Variable moved | Use `&` to borrow |
| Cannot borrow as mutable | Use `let mut` |
| Conflicting borrows | Don't hold references during mutation |
| Missing type annotation | Add `: Type` |
| No matching method | Implement the trait or import it |
| Expected &str, got String | Use `&string[..]` or `&string` |

## Helpful Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **Rust Documentation**: https://doc.rust-lang.org/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Cargo**: https://doc.rust-lang.org/cargo/

## Command Cheat Sheet

```bash
cargo new project              # Create new project
cargo build                    # Build project
cargo run                      # Build and run
cargo test                     # Run tests
cargo check                    # Check without building
cargo fmt                      # Format code
cargo clippy                   # Lint code
cargo doc --open               # Generate documentation
rustc --version                # Check Rust version
```

---

**Happy coding in Rust!** 🦀
