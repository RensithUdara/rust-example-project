# Chapter 2: Variables & Data Types 📦

## Variables Basics

In Rust, variables are **immutable by default**. This is a safety feature!

```rust
let x = 5;      // Immutable - cannot change
let mut y = 5;  // Mutable - can change
```

### Immutability Example

```rust
fn main() {
    let x = 5;
    println!("{}", x);  // 5
    
    x = 6;  // ❌ ERROR: Cannot reassign immutable variable
    println!("{}", x);
}
```

### Mutability Example

```rust
fn main() {
    let mut x = 5;
    println!("{}", x);  // 5
    
    x = 6;  // ✅ OK - x is mutable
    println!("{}", x);  // 6
}
```

## Why Immutability by Default?

```
Immutable = Safer Code = Fewer Bugs ✅

When you make something mutable, it's explicit:
"I intend this to change" ⚠️

When you see a variable, you know it won't mysteriously
change in other parts of your program
```

## Type System

Rust is **statically typed** - every variable has a type known at compile time.

### Explicit Type Declaration

```rust
let x: i32 = 5;        // 32-bit signed integer
let y: f64 = 3.14;     // 64-bit floating point
let name: &str = "Rust";  // String slice
let is_fun: bool = true;  // Boolean
```

### Type Inference

Rust can **infer** types automatically:

```rust
let x = 5;              // Rust knows this is i32
let y = 3.14;           // Rust knows this is f64
let name = "Rust";      // Rust knows this is &str
```

## Integer Types

### Signed Integers (can be negative)

| Type | Size | Range |
|------|------|-------|
| i8 | 1 byte | -128 to 127 |
| i16 | 2 bytes | -32,768 to 32,767 |
| i32 | 4 bytes | -2.1B to 2.1B |
| i64 | 8 bytes | -9.2 quintillion to 9.2 quintillion |
| i128 | 16 bytes | Huge range |
| isize | Platform dependent | Depends on processor (32/64 bit) |

### Unsigned Integers (non-negative only)

| Type | Size | Range |
|------|------|-------|
| u8 | 1 byte | 0 to 255 |
| u16 | 2 bytes | 0 to 65,535 |
| u32 | 4 bytes | 0 to 4.2B |
| u64 | 8 bytes | 0 to 18.4 quintillion |
| u128 | 16 bytes | Very large |
| usize | Platform dependent | Depends on processor |

**Default:** `i32` for integers (when type not specified)

### Integer Literals

```rust
let decimal = 98_222;       // Decimal with underscores (for readability)
let hex = 0xff;             // Hexadecimal
let octal = 0o77;           // Octal
let binary = 0b1111_0000;   // Binary
let byte = b'A';            // Single byte (ASCII character)
```

## Floating-Point Types

| Type | Size | Precision |
|------|------|-----------|
| f32 | 4 bytes | ~6 decimal digits |
| f64 | 8 bytes | ~15 decimal digits |

**Default:** `f64` (more precision)

```rust
let x = 2.0;        // f64
let y: f32 = 2.0;   // f32
```

## Boolean Type

```rust
let t = true;
let f = false;
let is_true: bool = true;
```

## Character Type

```rust
let c = 'z';
let z: char = 'ℤ';
let heart_eyed_cat = '😻';

// Note: 'c' is char (1 character, Unicode)
// Not to be confused with "string" (multiple bytes)
```

## String Types

### &str (String Slice) - Fixed size, immutable

```rust
let s = "Hello";  // &str - immutable string slice
```

### String - Growable, heap-allocated

```rust
let mut s = String::from("Hello");
s.push_str(" World");
println!("{}", s);  // "Hello World"
```

### Difference

```
&str: "static", known at compile time, immutable
String: dynamic, allocated at runtime, mutable
```

## Tuples

A collection of values with different types:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// Access by position
println!("{}", tup.0);  // 500
println!("{}", tup.1);  // 6.4
println!("{}", tup.2);  // 1

// Destructuring
let (x, y, z) = tup;
println!("x: {}, y: {}, z: {}", x, y, z);
```

### Empty Tuple (Unit Type)

```rust
let unit = ();  // Represents "nothing"
```

This is useful when a function doesn't return anything meaningful.

## Arrays

Collections of values with the same type, **fixed size**:

```rust
let a = [1, 2, 3, 4, 5];  // Array of 5 integers

// Explicit type and size
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Initialize array with all same values
let a = [3; 5];  // [3, 3, 3, 3, 3]

// Access elements
println!("{}", a[0]);  // 1
println!("{}", a[4]);  // 5
```

## Shadowing

You can declare a new variable with the same name:

```rust
let x = 5;
let x = x + 1;      // x is now 6
let x = x * 2;      // x is now 12

println!("{}", x);  // 12
```

This is **different** from assignment because:

```rust
let mut x = 5;
x = x + 1;  // Reassignment

vs

let x = 5;
let x = x + 1;  // Shadowing (creates new variable)
```

Shadowing even allows type changes:

```rust
let x = "5";
let x = x.len();  // Now x is i32
```

## Type Casting

Convert between types using `as`:

```rust
let x = 5i32;
let y = x as i64;       // from i32 to i64

let x = 3.6f64;
let y = x as i32;       // from f64 to i32 (3, truncates decimal)
```

## Constants vs Variables

```rust
const MAX_POINTS: u32 = 100_000;  // Constant - UPPERCASE_NAME
let x = 5;                         // Variable - can be shadowed
```

Constants:
- Must have explicit type
- Can be used in global scope
- Value is fixed and known at compile time
- Cannot be reassigned

## Summary

| Concept | Usage |
|---------|-------|
| Immutable | `let x = 5;` (default) |
| Mutable | `let mut x = 5;` |
| Type Annotation | `let x: i32 = 5;` |
| Type Inference | `let x = 5;` (Rust figures it out) |
| Constants | `const MAX: u32 = 100;` |
| Shadowing | `let x = 5; let x = 6;` |

**Key Takeaway:** Immutability by default leads to safer, more predictable code.
