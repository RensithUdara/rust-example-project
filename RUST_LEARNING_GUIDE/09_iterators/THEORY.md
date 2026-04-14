# Chapter 9: Iterators & Closures 🔄

## Iterators

An iterator is an object that allows you to iterate over a sequence:

```rust
let v = vec![1, 2, 3];
let iter = v.iter();  // Creates an iterator

for number in iter {
    println!("{}", number);
}
```

### Adapter Methods (Return Iterator)

**`iter()`** - Reference iterator (non-consuming)
```rust
let v = vec![1, 2, 3];
for &num in v.iter() {
    println!("{}", num);
}
// v still valid
```

**`iter_mut()`** - Mutable reference iterator
```rust
let mut v = vec![1, 2, 3];
for num in v.iter_mut() {
    *num += 1;
}
// v is [2, 3, 4]
```

**`into_iter()`** - Consuming iterator (takes ownership)
```rust
let v = vec![1, 2, 3];
for num in v.into_iter() {
    println!("{}", num);
}
// v no longer valid
```

### Iterator Adaptor Methods

These transform iterators (lazy - don't execute until consumed):

**`map()`** - Transform each element
```rust
let v = vec![1, 2, 3];
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
// [2, 4, 6]
```

**`filter()`** - Keep elements matching predicate
```rust
let v = vec![1, 2, 3, 4, 5];
let evens: Vec<i32> = v.iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x)
    .collect();
// [2, 4]
```

**`take(n)`** - Take first n elements
```rust
let v = (1..10).take(3).collect::<Vec<_>>();
// [1, 2, 3]
```

**`skip(n)`** - Skip first n elements
```rust
let v: Vec<i32> = (1..5).skip(2).collect();
// [3, 4]
```

### Consumer Methods (Execute iterator)

**`collect()`** - Collect into collection
```rust
let v: Vec<i32> = (1..=5).collect();
```

**`sum()`** - Sum all elements
```rust
let total: i32 = (1..=5).sum();
```

**`count()`** - Count elements
```rust
let count = (1..=5).count();
```

**`find()`** - Find first matching element
```rust
let result = (1..10).find(|x| x == &5);
// Some(5)
```

**`any()`** - Check if any match
```rust
let has_even = vec![1, 2, 3].iter().any(|x| x % 2 == 0);
```

**`all()`** - Check if all match
```rust
let all_positive = vec![1, 2, 3].iter().all(|x| x > &0);
```

### Chaining Iterators

```rust
(1..10)
    .filter(|x| x % 2 == 0)    // Keep even: 2, 4, 6, 8
    .map(|x| x * x)             // Square: 4, 16, 36, 64
    .take(2)                    // First 2: 4, 16
    .sum::<i32>()               // Sum: 20
```

## Closures

Anonymous functions that can capture their environment:

```rust
let add_one = |x| x + 1;
println!("{}", add_one(5));  // 6
```

### Closure Types

**Type Inference**: Rust infers closure types from usage

```rust
let closure = |x| x + 1;
let result = closure(5);    // Rust knows x is i32
```

**Explicit Types**: You can specify types

```rust
let closure = |x: i32| -> i32 { x + 1 };
println!("{}", closure(5));  // 6
```

### Capturing Environment

Closures can capture variables from their scope:

```rust
let y = 4;

let add_y = |x| x + y;  // Captures y
println!("{}", add_y(5));  // 9
```

### Borrowing vs Moving

**Immutable Borrow** (default):
```rust
let s = String::from("hello");
let borrow = || println!("{}", s);  // Borrows s
borrow();
println!("{}", s);  // Still valid
```

**Mutable Borrow**:
```rust
let mut s = String::from("hello");
let mut append = || s.push_str(" world");
append();
println!("{}", s);  // "hello world"
```

**Move**:
```rust
let s = String::from("hello");
let take = move || println!("{}", s);  // Takes ownership
take();
// println!("{}", s);  // Error: s was moved
```

### Closure Traits

Closures implement one of three traits:

| Trait | Captures | Multiple Calls |
|-------|----------|----------------|
| `Fn` | Immutable borrow | Yes |
| `FnMut` | Mutable borrow | Yes |
| `FnOnce` | Takes ownership | Once only |

```rust
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

let closure = |x| x * 2;
println!("{}", apply(closure, 5));  // 10
```

## Iterator Examples

### Combining Methods

```rust
let numbers = vec![1, 2, 3, 4, 5];

let result = numbers.iter()
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .sum::<i32>();
// 2*2 + 4*4 = 4 + 16 = 20
```

### Finding Elements

```rust
let v = vec![1, 2, 3, 4, 5];

if let Some(found) = v.iter().find(|x| *x == 3) {
    println!("Found: {}", found);
}
```

### Grouping

```rust
let v = vec![1, 2, 3, 4, 5];

let (evens, odds): (Vec<_>, Vec<_>) = v.iter()
    .partition(|x| *x % 2 == 0);
```

## Lazy Evaluation

Iterators are lazy - they don't execute until consumed:

```rust
let v = vec![1, 2, 3];

// This doesn't execute anything:
let _iter = v.iter().map(|x| {
    println!("Mapping: {}", x);
    x * 2
});

// Nothing printed yet!

// Now it executes:
let _result: Vec<_> = _iter.collect();
// Now it prints!
```

## Performance

Iterators are often **faster** than manual loops:
- Zero-cost abstractions
- Compiler optimizations

```rust
// Both do the same thing, iterator is often faster
let v = vec![1, 2, 3, 4, 5];

// Iterator approach
let sum1: i32 = v.iter().filter(|x| x % 2 == 0).sum();

// Manual loop
let mut sum2 = 0;
for num in &v {
    if num % 2 == 0 {
        sum2 += num;
    }
}
```

## Summary

| Method | Type | Purpose |
|--------|------|---------|
| `iter()` | Adapter | Immutable iteration |
| `iter_mut()` | Adapter | Mutable iteration |
| `into_iter()` | Adapter | Consuming iteration |
| `map()` | Adapter | Transform elements |
| `filter()` | Adapter | Select elements |
| `collect()` | Consumer | Collect into collection |
| `sum()` | Consumer | Sum elements |
| `count()` | Consumer | Count elements |
| Closure | Code | Anonymous function |

**Key Takeaway:** Iterators and closures are idiomatic Rust - they're powerful, expressive, and often faster!
