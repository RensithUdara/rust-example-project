# Chapter 10: Traits & Generics 🎭

## Generics

Generics allow you to write code that works with many types:

```rust
fn print_it<T>(val: T) {
    println!("Value: {:?}", val);  // Requires Debug trait
}

print_it(5);
print_it("hello");
```

### Generic Functions

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}
```

### Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

let int_point = Point { x: 5, y: 10 };
let float_point = Point { x: 1.0, y: 4.0 };
```

### Generic with Multiple Types

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

let pair = Pair {
    first: 5,
    second: "hello",
};
```

### Generic Methods

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {  // Only for f32
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

## Traits

A trait defines behavior that types can implement:

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f32,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}
```

### Common Traits

| Trait | Purpose | Method |
|-------|---------|--------|
| `Debug` | Print for debugging | `{:?}` |
| `Display` | Print for users | `{}` |
| `Clone` | Create a copy | `clone()` |
| `Copy` | Automatic copy | (automatic) |
| `PartialEq` | Compare equality | `==` |
| `Eq` | Full equality | (marker) |
| `PartialOrd` | Compare order | `<`, `>` |
| `Ord` | Full ordering | (marker) |
| `Default` | Default value | `Default::default()` |
| `From<T>` | Convert from T | `T.into()` |
| `Into<T>` | Convert to T | `value.into()` |

### Implementing Display

```rust
use std::fmt;

struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}

let p = Person {
    name: String::from("Alice"),
    age: 30,
};
println!("{}", p);  // "Alice is 30 years old"
```

### Trait Methods with Default Implementation

```rust
trait Animal {
    fn species(&self) -> &str;
    
    fn sound(&self) -> &str {
        "Some sound"  // Default implementation
    }
    
    fn describe(&self) {
        println!("{} says {}", self.species(), self.sound());
    }
}

struct Dog;

impl Animal for Dog {
    fn species(&self) -> &str {
        "Dog"
    }
    
    fn sound(&self) -> &str {
        "Woof!"
    }
}

let dog = Dog;
dog.describe();  // "Dog says Woof!"
```

### Trait Bounds

Specify that a generic must implement certain traits:

```rust
// Generic T must implement Display
fn print_item<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

// Multiple trait bounds
fn process<T: Clone + Display>(item: T) {
    println!("{}", item.clone());
}

// Using where clause
fn complex<T, U>(t: T, u: U)
where
    T: Clone + Display,
    U: Debug,
{
    println!("{:?}", u);
}
```

### Combining Multiple Traits

```rust
trait Drawable {
    fn draw(&self);
}

trait Movable {
    fn move_to(&mut self, x: i32, y: i32);
}

// Implement both traits
struct Shape {
    x: i32,
    y: i32,
}

impl Drawable for Shape {
    fn draw(&self) {
        println!("Drawing at ({}, {})", self.x, self.y);
    }
}

impl Movable for Shape {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}
```

### Trait Objects

Use trait objects when you need different types with same behavior:

```rust
trait Animal {
    fn sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn sound(&self) { println!("Woof!"); }
}

impl Animal for Cat {
    fn sound(&self) { println!("Meow!"); }
}

// Vector of different types implementing same trait
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
];

for animal in animals {
    animal.sound();
}
```

### Returning Trait Objects

```rust
fn get_animal(kind: &str) -> Box<dyn Animal> {
    if kind == "dog" {
        Box::new(Dog)
    } else {
        Box::new(Cat)
    }
}
```

## Commonly Used Traits

### Clone

```rust
#[derive(Clone)]
struct User {
    name: String,
}

let u1 = User { name: "Alice".to_string() };
let u2 = u1.clone();  // u1 still valid
```

### Default

```rust
#[derive(Default)]
struct Config {
    host: String,
    port: u16,
}

let config = Config::default();
```

### From/Into

```rust
impl From<&str> for Person {
    fn from(s: &str) -> Person {
        Person {
            name: s.to_string(),
        }
    }
}

let p: Person = "Alice".into();
```

### Iterator trait

```rust
trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<u32> {
        self.count += 1;
        if self.count <= 5 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

## When to Use Generics vs Traits

| Generics | Traits |
|----------|--------|
| Exact type known | Type flexible |
| Monomorphization (code duplication) | Dynamic dispatch |
| Compile-time | Runtime polymorphism |
| Faster | Slightly slower |
| `fn<T>()` | `fn(T: dyn Trait)` |

## Summary

| Concept | Purpose |
|---------|---------|
| Generic | Code works with many types |
| Trait | Define shared behavior |
| Trait bound | Restrict generics |
| Trait object | Dynamic polymorphism |
| Impl | Implement trait for type |
| Default trait | Automate implementations |

**Key Takeaway:** Generics and traits are Rust's way of writing flexible, reusable code!
