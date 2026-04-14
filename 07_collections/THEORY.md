# Chapter 7: Collections 📚

Collections are data structures that hold multiple values.

## Vectors (Vec<T>)

Similar to arrays but **resizable**:

```rust
// Create a new empty vector
let mut v: Vec<i32> = Vec::new();

// Using macro shorthand
let mut v = vec![1, 2, 3];

// Add elements
v.push(4);
v.push(5);

// Access elements
println!("{}", v[0]);  // 1

// Remove elements
v.pop();  // Removes last element

// Length
println!("{}", v.len());

// Iterate
for num in &v {
    println!("{}", num);
}

// Iterate mutably
for num in &mut v {
    *num += 10;
}
```

### Vector Methods

```rust
let v = vec![1, 2, 3, 4, 5];

v.first();              // Option<&T>
v.last();               // Option<&T>
v.get(2);               // Option<&T>
v.contains(&3);         // bool
v.starts_with(&[1, 2]); // bool
v.ends_with(&[4, 5]);   // bool
v.reverse();            // Reverse in place
v.sort();               // Sort in place
v.len();                // Number of elements
v.is_empty();           // bool
```

### Bounds Checking

```rust
let v = vec![1, 2, 3];

// Panics if out of bounds
println!("{}", v[10]);  // ❌ PANICS

// Safe - returns Option
match v.get(10) {
    Some(val) => println!("{}", val),
    None => println!("Out of bounds"),
}
```

## HashMaps (Key-Value Storage)

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

// Insert
map.insert("name", "Alice");
map.insert("age", "30");

// Get
if let Some(value) = map.get("name") {
    println!("Name: {}", value);
}

// Check contains
if map.contains_key("age") {
    println!("Has age");
}

// Remove
map.remove("age");

// Iterate
for (key, value) in &map {
    println!("{}: {}", key, value);
}

// Map length
println!("{}", map.len());
```

### Other HashMap Methods

```rust
let mut map = HashMap::new();
map.insert("a", 1);

// Get or insert
*map.entry("b").or_insert(0) += 1;

// Get mut
if let Some(val) = map.get_mut("a") {
    *val += 10;
}

// Clear
map.clear();

// Keys and values
for key in map.keys() { }
for val in map.values() { }
```

## HashMap with Custom Types

```rust
#[derive(Hash, Eq, PartialEq)]
struct User {
    id: u32,
}

let mut users = HashMap::new();
users.insert(User { id: 1 }, "Alice");
```

## Strings (Revisited)

```rust
// Create strings
let s1 = String::new();
let s2 = String::from("hello");
let s3 = "hello".to_string();

// Concatenate
let s4 = "hello".to_string() + " " + "world";
let s5 = format!("{} {}", "hello", "world");

// Methods
s2.len();           // Byte length
s2.chars().count(); // Character count
s2.contains("ell");
s2.starts_with("he");
s2.replace("l", "L");
```

## BTreeMap

Ordered key-value storage (like HashMap but sorted):

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(3, "three");
map.insert(1, "one");
map.insert(2, "two");

// Iterates in sorted order
for (k, v) in &map {
    println!("{}: {}", k, v);  // 1:one, 2:two, 3:three
}
```

## HashSet

Unique value collection:

```rust
use std::collections::HashSet;

let mut set = HashSet::new();
set.insert("apple");
set.insert("banana");
set.insert("apple");  // Won't add duplicate

println!("{}", set.len());  // 2

// Set operations
if set.contains("apple") {
    println!("Has apple");
}
```

## VecDeque (Double-ended Queue)

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::new();
deque.push_back(1);
deque.push_front(0);

deque.pop_back();   // Removes from end
deque.pop_front();  // Removes from front
```

## Collection Comparison

| Collection | Type | Use Case |
|-----------|------|----------|
| Vec<T> | Ordered, resizable | List of items |
| HashMap<K,V> | Unordered key-value | Fast lookup |
| BTreeMap<K,V> | Ordered key-value | Sorted keys |
| HashSet<T> | Unique values | Membership test |
| BTreeSet<T> | Ordered unique | Sorted unique |
| VecDeque<T> | Double-ended | Queue/deque |
| String | Text data | Unicode strings |

## Summary

| Operation | Vec | HashMap | HashSet |
|-----------|-----|---------|---------|
| Insert | `push()` | `insert()` | `insert()` |
| Remove | `pop()` | `remove()` | `remove()` |
| Access | `[i]` or `get()` | `get()` | `contains()` |
| Iterate | `for x in &v` | `for (k,v) in &m` | `for x in &s` |
| Length | `len()` | `len()` | `len()` |

**Key Takeaway:** Use collections to manage multiple related values efficiently!
