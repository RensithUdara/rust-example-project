# Chapter 6: Pattern Matching 🎯

## The Match Statement

Pattern matching is Rust's powerful control flow operator:

```rust
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    _ => default_expression,
}
```

### Basic Example

```rust
let number = 3;

match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Other"),  // Catch-all pattern
}
```

### With Enums

```rust
enum Status {
    Active,
    Inactive,
}

let status = Status::Active;

match status {
    Status::Active => println!("User is active"),
    Status::Inactive => println!("User is inactive"),
}
```

## Extracting Values from Patterns

### From Enums with Data

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::Write(String::from("hello"));

match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move: ({}, {})", x, y),
    Message::Write(text) => println!("Write: {}", text),
    Message::ChangeColor(r, g, b) => println!("Color: RGB({}, {}, {})", r, g, b),
}
```

### From Tuples

```rust
let tup = (0, -2, 3);

match tup {
    (0, y, z) => println!("First is 0, y={}, z={}", y, z),
    (x, 0, z) => println!("Second is 0"),
    _ => println!("Other"),
}
```

### From Structs

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };

match p {
    Point { x, y: 0 } => println!("On x-axis at x={}", x),
    Point { x: 0, y } => println!("On y-axis at y={}", y),
    Point { x, y } => println!("At ({}, {})", x, y),
}
```

## Option and Result Patterns

### Option Pattern

```rust
let opt = Some(5);

match opt {
    Some(x) => println!("Value: {}", x),
    None => println!("No value"),
}

// If let - for single case
if let Some(x) = opt {
    println!("Value: {}", x);
}
```

### Result Pattern

```rust
let result = divide(10, 2);

match result {
    Ok(x) => println!("Result: {}", x),
    Err(e) => println!("Error: {}", e),
}

// If let
if let Ok(x) = result {
    println!("Success: {}", x);
}
```

## Pattern Matching Features

### Ignore Values with `_`

```rust
let (x, _, z) = (1, 2, 3);
println!("{}, {}", x, z);  // Only use x and z
```

### Ignore Remaining with `..`

```rust
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

let point = Point3D { x: 1, y: 2, z: 3 };

match point {
    Point3D { x, .. } => println!("x is {}", x),
}
```

### Range Patterns

```rust
let x = 5;

match x {
    1..=3 => println!("One through three"),
    4..=6 => println!("Four through six"),
    _ => println!("Other"),
}

// Also works with char
let letter = 'c';

match letter {
    'a'..='d' => println!("Early alphabet"),
    'e'..='z' => println!("Late alphabet"),
    _ => println!("Not a letter"),
}
```

### Multiple Patterns

```rust
let x = 1;

match x {
    1 | 2 => println!("One or two"),
    3..=5 => println!("Three to five"),
    _ => println!("Other"),
}
```

### Guard Clauses

Additional condition in a match arm:

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("Less than 5: {}", x),
    Some(x) => println!("Greater than or equal to 5: {}", x),
    None => println!("None"),
}
```

### Complex Guards

```rust
let (x, y) = (4, false);

match (x, y) {
    (4..=6, _) if y => println!("x between 4-6 and y is true"),
    (_, true) => println!("y is true"),
    _ => println!("Other"),
}
```

## `if let` - Shorthand Matching

When you only care about one specific pattern:

```rust
// Instead of:
match opt {
    Some(x) => println!("Value: {}", x),
    _ => {},  // Do nothing for other cases
}

// Use:
if let Some(x) = opt {
    println!("Value: {}", x);
}

// Can add else:
if let Some(x) = opt {
    println!("Some: {}", x);
} else if let None = opt {
    println!("None");
}
```

## `while let` - Pattern Matching Loop

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

## All Pattern Types

| Pattern | Example | Use Case |
|---------|---------|----------|
| Literal | `5`, `"hello"` | Match exact values |
| Variable | `x` | Bind to variable |
| Wildcard | `_` | Ignore value |
| Range | `1..=5` | Match ranges |
| Multiple | `1 \| 2` | Match any of values |
| Struct | `Point { x, y }` | Destructure struct |
| Enum | `Option::Some(x)` | Destructure enum |
| Slice | `&[first, .., last]` | Match slice patterns |
| Guard | `if x > 5` | Additional condition |

## Best Practices

✅ **DO:**
- Use `match` for exhaustive checking
- Use `if let` when you only care about one case
- Use guard clauses for complex logic
- Always handle all cases (use `_` as fallback)
- Extract variables in patterns

❌ **DON'T:**
- Forget the `_` catch-all pattern (compiler will warn)
- Use deeply nested matches (refactor instead)
- Use `match` when `if let` is clearer
- Match on the same value multiple times

## Summary

Pattern matching is Rust's way of:
- Extracting data from complex types
- Handling multiple cases safely
- Ensuring exhaustive handling of all possibilities

**Key Takeaway:** Pattern matching makes your code safer and more expressive!
