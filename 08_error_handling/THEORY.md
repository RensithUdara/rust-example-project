# Chapter 8: Error Handling 🛡️

Rust has two styles of error handling:

## Panics - Unrecoverable Errors

For errors you can't recover from, use `panic!`:

```rust
panic!("Something terrible happened!");

// Also occurs on:
let v = vec![1, 2, 3];
v[99];  // Panic: index out of bounds
```

Use panics for:
- Testing
- Prototyping
- When a situation should never happen (impossible state)

## Result<T, E> - Recoverable Errors

For errors you can recover from, use `Result`:

```rust
enum Result<T, E> {
    Ok(T),      // Success with value T
    Err(E),     // Failure with error E
}
```

### Basic Example

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// Using match
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

### `unwrap()` - Extract Value or Panic

```rust
let result = divide(10, 2);
let value = result.unwrap();  // Gets Ok value or panics on Err

// Same as:
let value = match result {
    Ok(v) => v,
    Err(_) => panic!("Called unwrap on Err"),
};
```

### `expect()` - Unwrap with Custom Message

```rust
let result = divide(10, 0);
let value = result.expect("Division failed!");  // Panics with message
```

### `unwrap_or()` - Default Value

```rust
let result = divide(10, 0);
let value = result.unwrap_or(0);  // Returns 0 if error
```

### `map()` - Transform Success Value

```rust
let result = divide(10, 2);

let double = result.map(|x| x * 2);
// If Ok(5), returns Ok(10)
// If Err, returns Err unchanged
```

### `and_then()` - Chain Operations

```rust
fn convert_to_string(n: i32) -> Result<String, String> {
    if n < 0 {
        Err(String::from("Negative number"))
    } else {
        Ok(n.to_string())
    }
}

fn process(n: i32) -> Result<String, String> {
    divide(n, 2)
        .and_then(|half| convert_to_string(half))
}
```

### `?` Operator - Early Return for Errors

The `?` operator extracts the Ok value or returns the Err:

```rust
fn process(n: i32) -> Result<String, String> {
    let half = divide(n, 2)?;  // Return if error
    let string = convert_to_string(half)?;  // Return if error
    Ok(string)
}

// Same as:
fn process(n: i32) -> Result<String, String> {
    let half = match divide(n, 2) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let string = match convert_to_string(half) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    Ok(string)
}
```

### Reading Files

```rust
use std::fs;
use std::io;

fn read_config() -> Result<String, io::Error> {
    fs::read_to_string("config.txt")
}

// Using ?
fn read_config_with_fallback() -> Result<String, io::Error> {
    let content = fs::read_to_string("config.txt")?;
    Ok(content)
}
```

## Custom Error Types

Create your own error type:

```rust
use std::fmt;

#[derive(Debug)]
enum CalculatorError {
    DivisionByZero,
    InvalidNumber(String),
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "Division by zero"),
            CalculatorError::InvalidNumber(n) => write!(f, "Invalid number: {}", n),
        }
    }
}

impl std::error::Error for CalculatorError {}

fn parse_number(s: &str) -> Result<i32, CalculatorError> {
    s.parse()
        .map_err(|_| CalculatorError::InvalidNumber(s.to_string()))
}
```

## Option<T> vs Result<T, E>

| Type | Use Case | Success | Failure |
|------|----------|---------|---------|
| `Option<T>` | Value might be absent | `Some(T)` | `None` |
| `Result<T, E>` | Operation might fail | `Ok(T)` | `Err(E)` |

Use Option when:
- Value is optional (might not exist)
- No error information needed

Use Result when:
- Operation can fail
- Need error details

## Handling Multiple Error Types

```rust
use std::num::ParseIntError;
use std::io;

fn read_and_parse() -> Result<i32, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("file.txt")?;
    let number: i32 = content.trim().parse()?;
    Ok(number)
}
```

## Best Practices

✅ **DO:**
- Use `Result` for fallible operations
- Use `?` operator for chaining operations
- Use `panic!` only for impossible states
- Provide context in error messages
- Use custom error types for libraries

❌ **DON'T:**
- Use `unwrap()` in production code
- Ignore errors by returning `None` or `0`
- Create unclear error messages
- Panic when you shouldn't

## Error Handling Flow

```
Try operation
    ⬇
Success? ──→ Return Ok(value)
    ⬇
   No
    ⬇
Recoverable? ──→ Return Err(error)
    ⬇
   No
    ⬇
Unrecoverable ──→ panic!()
```

## Summary

| Method | Purpose |
|--------|---------|
| `unwrap()` | Get value or panic |
| `expect(msg)` | Get value or panic with message |
| `unwrap_or(def)` | Get value or default |
| `map(f)` | Transform Ok value |
| `and_then(f)` | Chain operations |
| `?` | Return error early |

**Key Takeaway:** Rust forces you to handle errors explicitly, making your code safer and more reliable!
