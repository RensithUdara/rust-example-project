// ============ CHAPTER 7: COLLECTIONS ============

use std::collections::{HashMap, HashSet, BTreeMap};

fn main() {
    println!("=== VECTORS ===\n");
    
    // Create vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vector: {:?}", v);
    
    // Using macro
    let mut nums = vec![10, 20, 30];
    nums.push(40);
    println!("Numbers: {:?}", nums);
    
    // Access elements
    println!("First: {}", nums[0]);
    println!("Using get: {:?}", nums.get(1));
    
    // Vector methods
    println!("Length: {}", nums.len());
    println!("Contains 20: {}", nums.contains(&20));
    println!();
    
    
    println!("=== VECTOR ITERATION ===\n");
    
    let v = vec![1, 2, 3, 4, 5];
    
    // Immutable iteration
    for num in &v {
        println!("Immutable: {}", num);
    }
    
    // Mutable iteration
    let mut v2 = vec![1, 2, 3];
    for num in &mut v2 {
        *num *= 2;
    }
    println!("After doubling: {:?}", v2);
    
    // Consuming iteration
    let v3 = vec![1, 2, 3];
    for num in v3 {
        println!("Consuming: {}", num);
    }
    // v3 is no longer valid
    println!();
    
    
    println!("=== HASHMAPS ===\n");
    
    let mut map = HashMap::new();
    
    // Insert
    map.insert("name", "Alice");
    map.insert("age", "30");
    map.insert("city", "NYC");
    
    // Get
    if let Some(name) = map.get("name") {
        println!("Name: {}", name);
    }
    
    // Check contains
    if map.contains_key("age") {
        println!("Has age key");
    }
    
    // Length
    println!("Map size: {}", map.len());
    
    // Remove
    map.remove("city");
    println!("After removal: {:?}", map);
    println!();
    
    
    println!("=== HASHMAP ITERATION ===\n");
    
    let mut scores = HashMap::new();
    scores.insert("Alice", 100);
    scores.insert("Bob", 85);
    scores.insert("Charlie", 92);
    
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
    println!();
    
    
    println!("=== HASHMAP WITH CUSTOM KEYS ===\n");
    
    let mut user_ids = HashMap::new();
    user_ids.insert(1, "Alice");
    user_ids.insert(2, "Bob");
    user_ids.insert(3, "Charlie");
    
    for (id, name) in &user_ids {
        println!("ID {}: {}", id, name);
    }
    println!();
    
    
    println!("=== HASHMAP COUNT WORDS ===\n");
    
    let text = "hello world hello rust world";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("Word counts: {:?}", word_count);
    println!();
    
    
    println!("=== BTREEMAP (ORDERED) ===\n");
    
    let mut map = BTreeMap::new();
    map.insert(3, "three");
    map.insert(1, "one");
    map.insert(2, "two");
    
    // Iterates in sorted order by key
    for (k, v) in &map {
        println!("{}: {}", k, v);  // 1:one, 2:two, 3:three
    }
    println!();
    
    
    println!("=== HASHSET ===\n");
    
    let mut set = HashSet::new();
    set.insert("apple");
    set.insert("banana");
    set.insert("apple");  // Won't add duplicate
    
    println!("Set: {:?}", set);
    println!("Length: {}", set.len());  // 2, not 3
    println!("Contains apple: {}", set.contains("apple"));
    
    // Iteration
    for item in &set {
        println!("Item: {}", item);
    }
    println!();
    
    
    println!("=== SET OPERATIONS ===\n");
    
    let set1: HashSet<_> = ["a", "b", "c"].iter().collect();
    let set2: HashSet<_> = ["b", "c", "d"].iter().collect();
    
    // Union
    let union: HashSet<_> = set1.union(&set2).collect();
    println!("Union: {:?}", union);
    
    // Intersection
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    println!("Intersection: {:?}", intersection);
    
    // Difference
    let difference: HashSet<_> = set1.difference(&set2).collect();
    println!("Difference: {:?}", difference);
    println!();
    
    
    println!("=== STRINGS ===\n");
    
    let s1 = String::new();
    let s2 = String::from("hello");
    let s3 = "world".to_string();
    
    let combined = format!("{} {}", s2, s3);
    println!("Combined: {}", combined);
    
    // String methods
    println!("Length: {}", combined.len());
    println!("Contains 'world': {}", combined.contains("world"));
    println!("Starts with 'hello': {}", combined.starts_with("hello"));
    
    // Replace
    let replaced = combined.replace("hello", "hi");
    println!("Replaced: {}", replaced);
    println!();
    
    
    println!("=== STRING ITERATION ===\n");
    
    let s = "hello";
    
    // Iterate over chars
    for c in s.chars() {
        println!("Char: {}", c);
    }
    
    // Iterate over bytes
    for b in s.bytes() {
        println!("Byte: {}", b);
    }
    println!();
    
    
    println!("=== VECTOR OF VECTORS ===\n");
    
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    
    println!("Matrix: {:?}", matrix);
    println!("Element [1][2]: {}", matrix[1][2]);  // 6
}
