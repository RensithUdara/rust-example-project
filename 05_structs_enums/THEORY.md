# Chapter 5: Structs & Enums 🏗️

## Structs - Grouping Related Data

A struct is a custom data type that groups related values together.

### Defining a Struct

```rust
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}
```

### Creating an Instance

```rust
let mut user = User {
    email: String::from("user@example.com"),
    username: String::from("rust_user"),
    active: true,
    age: 25,
};

// Access fields with dot notation
println!("{}", user.email);

// Modify fields (if mutable)
user.age = 26;
```

### Field Init Shorthand

```rust
fn create_user(email: String, username: String) -> User {
    User {
        email,      // Shorthand for email: email
        username,   // Shorthand for username: username
        active: true,
        age: 0,
    }
}
```

### Struct Update Syntax

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("another_user"),
    ..user1  // Copy remaining fields from user1
};
```

## Methods on Structs

Add functions (methods) to structs using `impl` blocks:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Constructor (convention)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Usage
let rect = Rectangle { width: 30, height: 50 };
println!("Area: {}", rect.area());
```

### `&self` vs `&mut self` vs `self`

```rust
impl Rectangle {
    fn area(&self) -> u32 {           // Borrow immutably (read only)
        self.width * self.height
    }
    
    fn set_width(&mut self, width: u32) {  // Borrow mutably (modify)
        self.width = width;
    }
    
    fn into_dimensions(self) -> (u32, u32) {  // Take ownership
        (self.width, self.height)
    }
}
```

## Tuple Structs

Structs without named fields:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

println!("{}", black.0);  // Access by index
```

## Unit Structs

Structs with no fields (useful for traits):

```rust
struct Marker;  // Empty struct

let marker = Marker;
```

## Enums - Multiple Possible Values

An enum lets you define a type that can be one of several variants:

```rust
enum Status {
    Active,
    Inactive,
    Pending,
}

let user_status = Status::Active;
```

### Enums with Associated Data

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::Write(String::from("Hello"));
```

### The Option Enum (Very Common!)

The `Option<T>` type handles nullable values safely:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

**Never use `null`** - use `Option` instead:

```rust
let some_number = Some(5);
let some_string = Some("a string");
let absent: Option<i32> = None;

// Extract the value
match some_number {
    Some(n) => println!("Number: {}", n),
    None => println!("No number"),
}

// Or use if let
if let Some(n) = some_number {
    println!("Number: {}", n);
}
```

### The Result Enum (Error Handling)

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// Usage
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

## Methods on Enums

```rust
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to {}, {}", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
        }
    }
}
```

## Deriving Traits

The `#[derive(...)]` attribute automatically implements traits:

```rust
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 0 };
println!("{:?}", p);  // Debug print
let p2 = p.clone();   // Clone
```

## Comparison: Structs vs Enums

| Structs | Enums |
|---------|-------|
| Group related data | Choose one variant |
| All fields exist | Only one variant active |
| "Product type" | "Sum type" |
| `struct User { name, age }` | `enum Status { Active, Inactive }` |

**Real World Example:**

```rust
struct User {
    username: String,
    email: String,
    status: UserStatus,  // Uses an enum
}

enum UserStatus {
    Active,
    Inactive,
    Banned { reason: String },
}
```

## Summary

| Concept | Purpose |
|---------|---------|
| Struct | Group related data together |
| `impl` block | Add methods to structs/enums |
| Method | Function that takes `&self`, `&mut self`, or `self` |
| Tuple struct | Struct without named fields |
| Unit struct | Struct with no fields |
| Enum | Type with multiple possible variants |
| `Option<T>` | Represents Some(T) or None |
| `Result<T, E>` | Represents Ok(T) or Err(E) |
| Derive | Auto-implement common traits |

**Key Takeaway:** 
Structs and enums are the foundation of Rust's type system. They let you create custom types that model your domain precisely!
