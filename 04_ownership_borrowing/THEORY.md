# Chapter 4: Ownership & Borrowing ⭐ (THE MOST IMPORTANT!)

## The Ownership Problem

Most languages use either:
1. **Garbage Collector** (Python, Java, JavaScript) - Automatic memory management but slower
2. **Manual Memory Management** (C, C++) - Fast but prone to bugs (memory leaks, use-after-free)

Rust solves this with **Ownership Rules** - checked at **compile time**, not runtime!

## The Three Rules of Ownership

### Rule 1: Every value has exactly one owner

```
┌─────────────────────────────────┐
│  let x = String::from("hello"); │
│    ↑                            │
│    └── x is the owner           │
└─────────────────────────────────┘
```

### Rule 2: When the owner goes out of scope, the value is freed

```rust
{
    let s = String::from("hello");  // s owns the String
    // ... use s ...
}  // End of scope - s is freed automatically, memory cleaned up!
```

### Rule 3: Ownership can be transferred (moved)

```rust
let s1 = String::from("hello");
let s2 = s1;  // Ownership moved from s1 to s2

println!("{}", s1);  // ❌ ERROR! s1 no longer owns the String
println!("{}", s2);  // ✅ OK! s2 is the owner now
```

## Move vs Copy

### Move (for heap types)

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1's ownership moves to s2

println!("{}", s1);  // ❌ ERROR: s1 is no longer valid
```

### Copy (for stack types)

```rust
let x = 5;
let y = x;  // x is copied to y (both are valid)

println!("{}", x);  // ✅ OK
println!("{}", y);  // ✅ OK
```

**Types that Copy:** i32, f64, bool, char, and tuples of Copy types
**Types that Move:** String, Vec, HashMap (heap allocated)

## References & Borrowing

Instead of transferring ownership, you can **borrow** with references using `&`:

### Immutable References

```rust
let s1 = String::from("hello");
let s2 = &s1;  // Borrow s1, don't take ownership

println!("{}", s1);  // ✅ OK - s1 still owns it
println!("{}", s2);  // ✅ OK - s2 borrowed it
```

Think of it like borrowing a book from a friend:
- Your friend still owns the book
- You can read it (but can't write in it)
- You return it when done

### Why Borrow?

```rust
// Without borrowing (ownership transfer)
fn take_ownership(s: String) {
    println!("{}", s);
}  // s is freed here

let s = String::from("hello");
take_ownership(s);  // s is moved/freed
println!("{}", s);  // ❌ ERROR: s was freed

// With borrowing (reference)
fn borrow_string(s: &String) {
    println!("{}", s);
}  // s is not freed, just the reference ends

let s = String::from("hello");
borrow_string(&s);   // Borrow s
println!("{}", s);   // ✅ OK: s still valid
```

## Mutable References

You can borrow mutably to modify data:

```rust
fn main() {
    let mut s = String::from("hello");
    
    change_string(&mut s);  // Mutable borrow
    println!("{}", s);      // "hello world"
}

fn change_string(s: &mut String) {
    s.push_str(" world");
}
```

### BUT: The Borrow Checker Rules!

You can have **either**:
- Many immutable references, **OR**
- One mutable reference

```rust
let mut s = String::from("hello");

let r1 = &s;      // ✅ Immutable borrow
let r2 = &s;      // ✅ Immutable borrow
let r3 = &mut s;  // ❌ ERROR! Can't have mutable AND immutable

println!("{} {} {}", r1, r2, r3);
```

Valid:
```rust
let mut s = String::from("hello");

let r1 = &s;      // ✅ Immutable
let r2 = &s;      // ✅ Immutable
println!("{} {}", r1, r2);  // Use them

let r3 = &mut s;  // ✅ Mutable (no more immutable borrows)
println!("{}", r3);
```

### Why These Rules?

```
Preventing data races at compile time!

Race Condition Example (Forbidden in Rust):
┌─────────────────────────────┐
│ Thread 1: Read x            │ ← Reading
│ Thread 2: Write x           │ ← Writing at same time = CRASH!
│ Thread 1: Use x             │
└─────────────────────────────┘

Rust prevents this by ensuring:
- Multiple readers OK (immutable)
- One writer only (mutable)
- Never both at same time
```

## Slices

A slice is a reference to a portion of data:

```rust
let s = String::from("hello world");

let hello = &s[0..5];   // References "hello"
let world = &s[6..11];  // References "world"

println!("{}", hello);  // "hello"
println!("{}", world);  // "world"
```

### String Slices

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];  // Return string slice
        }
    }
    
    &s[..]  // Return whole string
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);  // "hello"
}
```

### Array Slices

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];  // [2, 3]
println!("{:?}", slice);
```

## Ownership Diagram

```
Stack vs Heap:

Stack (Copy types):
┌──────────┐
│ x: 5     │ ← i32, stored directly
│ y: 5     │ ← Copy of x, both valid
└──────────┘

Heap (Move types):
s1 owns String:
┌─────────────┐         Heap
│ ptr→−−−−−−−→┌──────────────┐
│ len: 5      │ "hello"      │
│ capacity: 5 │              │
└─────────────┘└──────────────┘

s2 owns String (s1 moved):
┌─────────────┐         Heap
│ ptr→−−−−−−−→┌──────────────┐
│ len: 5      │ "hello"      │
│ capacity: 5 │              │
└─────────────┘└──────────────┘
(s1 is now invalid!)

r = &s (reference/borrow):
┌──────────────┐
│ String s     │
│ ref r→−−−−−→┘  ← r points to s, doesn't own it
└──────────────┘
```

## Common Errors Explained

### Error 1: Use after move

```rust
let s1 = String::from("hello");
let s2 = s1;          // Move
println!("{}", s1);   // ❌ ERROR: s1 no longer owns it
```

**Fix:** Use references
```rust
let s1 = String::from("hello");
let s2 = &s1;        // Borrow instead of move
println!("{}", s1);  // ✅ OK
```

### Error 2: Conflicting references

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s;      // ❌ ERROR: Can't mix immutable and mutable
```

**Fix:** Don't use immutable refs anymore
```rust
let mut s = String::from("hello");
let r1 = &s;
println!("{}", r1);   // Use it

let r2 = &mut s;      // ✅ Now OK (r1 not used anymore)
r2.push_str("!");
```

### Error 3: Dangling reference

```rust
fn dangle() -> &String {  // ❌ ERROR: Lifetime issue
    let s = String::from("hello");
    &s  // Returns reference to s, but s is freed when function ends!
}
```

**Fix:** Return the String itself, not reference
```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // Returns ownership, always safe
}
```

## Summary

| Concept | Usage |
|---------|-------|
| Ownership | One owner, frees on scope end |
| Move | Transfer ownership |
| Copy | Duplicate for stack types |
| Immutable Borrow | `&x` - read-only |
| Mutable Borrow | `&mut x` - can modify |
| Slice | Reference to part of data |
| Borrow Rules | Many readers OR one writer |

## Key Takeaways

✅ Ownership enables memory safety at compile time
✅ Borrowing with `&` lets you use data without taking ownership
✅ The borrow checker prevents data races
✅ Understand Move vs Copy
✅ Mutable bindings (`mut`) vs Mutable references (`&mut`)

**This is what makes Rust unique and powerful!** 🦀
